use std::str::FromStr;

use anyhow::anyhow;
use mongodb::bson::Document;

use super::command::{self, QueryCommand};

/// transforms query into a list of commands
/// execute each command and push the resulting documents into a vector of agregation objects
///
/// example input: container camunda | sort DESC
/// output (json like):
/// ```json
/// [
///     {
///         "$match": {
///             "container_name": "camunda"
///         }
///     },
///     {
///         "$sort": {
///             "date": -1
///         }
///     }
/// ]
/// ```
///
/// # Examples
///
/// ```
/// use storage::query_lang::parse;
/// let result = parse::into_aggregation("container camunda | sort DESC");
/// assert!(result.is_ok());
/// ```
pub fn into_aggregation(query: &str) -> anyhow::Result<Vec<Document>> {
    let mut aggregation: Vec<Document> = vec![];
    let mut push = |doc| aggregation.push(doc);
    for pipe in query.split('|') {
        let mut pipe = pipe.split_whitespace();
        let Some(query_command) = pipe.next() else {
            return Err(anyhow!("No query command provided in pipe"));
        };
        let query = Query::from_str(query_command.trim())?;
        query.execute(pipe.collect(), &mut push)?;
    }
    Ok(aggregation)
}

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
    fn execute(&self, pipe: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
        match self {
            Query::Container(cmd) => cmd.execute(pipe, push),
            Query::Offset(cmd) => cmd.execute(pipe, push),
            Query::LogLevel(cmd) => cmd.execute(pipe, push),
            Query::Find(cmd) => cmd.execute(pipe, push),
            Query::Sort(cmd) => cmd.execute(pipe, push),
            Query::Take(cmd) => cmd.execute(pipe, push),
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
    use super::into_aggregation;

    #[test]
    fn can_parse_simple_query() {
        let cmds = into_aggregation("container auth-service | offset 10m | log_level INFO | find login | sort DESC | take 30");
        assert!(cmds.is_ok());
    }
}
