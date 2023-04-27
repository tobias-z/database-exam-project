use std::str::FromStr;

use anyhow::anyhow;
use mongodb::bson::Document;

use super::command::{self, QueryCommand};

pub fn parse_query(query: &str) -> anyhow::Result<Vec<Document>> {
    // transform query into a list of commands
    // execute each command and push the resulting documents into a vector of agregation objects
    let mut agregation = vec![];
    for pipe in query.split('|') {
        let mut pipe = pipe.split_whitespace();
        let Some(query_command) = pipe.next() else {
            return Err(anyhow!("No query command provided in pipe"));
        };
        let query = Query::from_str(query_command.trim())?;
        agregation.push(query.execute(pipe.collect())?);
    }
    Ok(agregation)
}

// container auth-service | offset 10m | log_level INFO | find "login" | contains "invalid" | sort date DESC | take 30
// TODO: command to combine commands together to a single pipe
// TODO: deep extend of documents function
enum Query {
    Container(command::ContainerCommand),
    Offset(command::OffsetCommand),
    LogLevel(command::LogLevelCommand),
    Find(command::FindCommand),
    Sort(command::SortCommand),
    Take(command::TakeCommand),
}

impl Query {
    fn execute(&self, pipe: Vec<&str>) -> anyhow::Result<Document> {
        match self {
            Query::Container(cmd) => cmd.execute(pipe),
            Query::Offset(cmd) => cmd.execute(pipe),
            Query::LogLevel(cmd) => cmd.execute(pipe),
            Query::Find(cmd) => cmd.execute(pipe),
            Query::Sort(cmd) => cmd.execute(pipe),
            Query::Take(cmd) => cmd.execute(pipe),
        }
    }
}

impl FromStr for Query {
    type Err = anyhow::Error;

    fn from_str(query_command: &str) -> Result<Self, Self::Err> {
        match query_command {
            "container" | "CONTAINER" => Ok(Query::Container(command::ContainerCommand)),
            "offset" | "OFFSET" => Ok(Query::Offset(command::OffsetCommand)),
            "log_level" | "LOG_LEVEL" => Ok(Query::LogLevel(command::LogLevelCommand)),
            "find" | "FIND" => Ok(Query::Find(command::FindCommand)),
            "sort" | "SORT" => Ok(Query::Sort(command::SortCommand)),
            "take" | "TAKE" => Ok(Query::Take(command::TakeCommand)),
            _ => Err(anyhow!("Unknown query command {}", query_command)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::parse_query;

    #[test]
    fn can_parse_simple_query() {
        let cmds = parse_query("container auth-service | offset 10m | log_level INFO | find login | sort date DESC | take 30");
        assert!(cmds.is_ok());
    }
}
