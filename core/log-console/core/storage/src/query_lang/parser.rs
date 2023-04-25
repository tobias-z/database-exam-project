use std::str::FromStr;

use anyhow::anyhow;
use mongodb::bson::{doc, DateTime, Document};

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

trait QueryCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document>;
}

// container auth-service | offset 10m | log_level INFO | find "login" | contains "invalid" | sort date DESC | take 30
// TODO: command to combine commands together to a single pipe
// TODO: deep extend of documents function
enum Query {
    Container(ContainerCommand),
    Offset(OffsetCommand),
    LogLevel(LogLevelCommand),
    Find(FindCommand),
    Sort(SortCommand),
    Take(TakeCommand),
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
            "container" | "CONTAINER" => Ok(Query::Container(ContainerCommand)),
            "offset" | "OFFSET" => Ok(Query::Offset(OffsetCommand)),
            "log_level" | "LOG_LEVEL" => Ok(Query::LogLevel(LogLevelCommand)),
            "find" | "FIND" => Ok(Query::Find(FindCommand)),
            "sort" | "SORT" => Ok(Query::Sort(SortCommand)),
            "take" | "TAKE" => Ok(Query::Take(TakeCommand)),
            _ => Err(anyhow!("Unknown query command {}", query_command)),
        }
    }
}

pub struct ContainerCommand;

impl QueryCommand for ContainerCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document> {
        let Some(container_name) = args.first() else {
            return Err(anyhow!("No container name provided in container query command"));
        };
        Ok(doc! {
            "$match": {
                "container_name": container_name
            }
        })
    }
}

pub struct OffsetCommand;

impl OffsetCommand {
    /// This method translates an input of e.g. '10ms' into the DateTime of now minus the provided
    /// range
    fn get_datetime_from_range(&self, range: &str) -> anyhow::Result<DateTime> {
        let now = DateTime::now();
        // minutes
        if range.ends_with('m') {
            let duration = range[0..range.len() - 1].parse::<i64>()?;
            Ok(DateTime::from_millis(
                now.timestamp_millis() - (duration * 60_000),
            ))
        // milliseconds
        } else if range.ends_with("ms") {
            let duration = range[0..range.len() - 2].parse::<i64>()?;
            Ok(DateTime::from_millis(now.timestamp_millis() - duration))
        // seconds
        } else if range.ends_with('s') {
            let duration = range[0..range.len() - 1].parse::<i64>()?;
            Ok(DateTime::from_millis(
                now.timestamp_millis() - (duration * 1000),
            ))
        // hours
        } else if range.ends_with('h') {
            let duration = range[0..range.len() - 1].parse::<i64>()?;
            Ok(DateTime::from_millis(
                now.timestamp_millis() - (duration * 3_600_000),
            ))
        // days
        } else if range.ends_with('d') {
            let duration = range[0..range.len() - 1].parse::<i64>()?;
            Ok(DateTime::from_millis(
                now.timestamp_millis() - (duration * 3_600_000 * 24),
            ))
        } else {
            Err(anyhow!(
                "Invalid datetime range provided {} to offset query command",
                range
            ))
        }
    }
}

impl QueryCommand for OffsetCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document> {
        let Some(time_arg) = args.first() else {
            return Err(anyhow!("No time arg provided in log level query command"));
        };
        Ok(doc! {
            "$match": {
                "date": {
                    "$gte": ["$date", self.get_datetime_from_range(time_arg)?]
                }
            }
        })
    }
}

pub struct LogLevelCommand;

impl QueryCommand for LogLevelCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document> {
        Ok(doc! {})
    }
}

pub struct FindCommand;

impl QueryCommand for FindCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document> {
        Ok(doc! {})
    }
}

pub struct SortCommand;

impl QueryCommand for SortCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document> {
        Ok(doc! {})
    }
}

pub struct TakeCommand;

impl QueryCommand for TakeCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document> {
        Ok(doc! {})
    }
}
