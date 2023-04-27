use anyhow::anyhow;
use mongodb::bson::{doc, DateTime, Document};

use crate::log_service;

pub trait QueryCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>>;
}

pub struct ContainerCommand;

impl QueryCommand for ContainerCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>> {
        let Some(container_name) = args.first() else {
            return Err(anyhow!("No container name provided in container query command"));
        };
        Ok(vec![doc! {
            "$match": {
                "container_name": container_name
            }
        }])
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
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>> {
        let Some(time_arg) = args.first() else {
            return Err(anyhow!("No time arg provided in log level query command"));
        };
        Ok(vec![doc! {
            "$match": {
                "date": {
                    "$gte": ["$date", self.get_datetime_from_range(time_arg)?]
                }
            }
        }])
    }
}

pub struct LogLevelCommand;

impl QueryCommand for LogLevelCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>> {
        let Some(log_level) = args.first() else {
            return Err(anyhow!("No log level provided in log_level query command"));
        };
        // map the given log level to the stored number for that log level
        match log_service::get_log_level(log_level) {
            Some(log_level) => Ok(vec![doc! {
                "$match": {
                    "level": log_level as i32,
                }
            }]),
            None => Err(anyhow!("invalid log level provided {}", log_level)),
        }
    }
}

pub struct FindCommand;

impl QueryCommand for FindCommand {
    /// This makes use of the a text index in mongodb
    /// https://www.mongodb.com/docs/manual/core/index-text/
    /// We index the message field as a text index.
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>> {
        if args.is_empty() {
            return Err(anyhow!("No search term provided to the find command"));
        }
        Ok(vec![
            doc! {
                "$text": {
                    "$search": args.join(" "),
                },
            },
            // sort the results by their score
            doc! {
                "$sort": {
                    "score": {
                        "$meta": "textScore"
                    }
                }
            },
        ])
    }
}

pub struct SortCommand;

impl QueryCommand for SortCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>> {
        let Some(direction) = args.first() else {
            return Err(anyhow!("No direction provided in sort query command"));
        };
        let direction = match *direction {
            "ASC" => 1,
            "DESC" => -1,
            _ => {
                return Err(anyhow!(
                    "Invalid direction provided '{}' expected ASC or DESC",
                    direction
                ))
            }
        };
        Ok(vec![doc! {
            "$sort": {
                "date": direction
            }
        }])
    }
}

pub struct TakeCommand;

impl QueryCommand for TakeCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Vec<Document>> {
        let Some(amount) = args.first() else {
            return Err(anyhow!("No amount provided in take query command"));
        };
        let Ok(amount) = amount.parse::<i32>() else {
            return Err(anyhow!("Expected the second parmater to the take query command, but got {}", amount));
        };
        Ok(vec![doc! {
            "$limit": amount
        }])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_map_container() {
        assert!(ContainerCommand.execute(vec!["something"]).is_ok());
        assert!(ContainerCommand.execute(vec![]).is_err());
    }

    #[test]
    fn can_map_offset() {
        assert!(OffsetCommand.execute(vec!["10ms"]).is_ok());
        assert!(OffsetCommand.execute(vec!["10m"]).is_ok());
        assert!(OffsetCommand.execute(vec!["10s"]).is_ok());
        assert!(OffsetCommand.execute(vec!["10h"]).is_ok());
        assert!(OffsetCommand.execute(vec!["10d"]).is_ok());

        assert!(OffsetCommand.execute(vec!["10j"]).is_err());
    }

    #[test]
    fn can_map_log_level() {
        assert!(LogLevelCommand.execute(vec!["INFO"]).is_ok());
        assert!(LogLevelCommand.execute(vec!["ERROR"]).is_ok());
        assert!(LogLevelCommand.execute(vec!["WARN"]).is_ok());
        assert!(LogLevelCommand.execute(vec!["TRACE"]).is_ok());
        assert!(LogLevelCommand.execute(vec!["DEBUG"]).is_ok());
        assert!(LogLevelCommand.execute(vec!["FATAL"]).is_ok());

        assert!(LogLevelCommand.execute(vec!["UNKNOWN"]).is_err());
    }

    #[test]
    fn can_map_find() {
        assert!(FindCommand.execute(vec!["hello", "world"]).is_ok());
        assert!(FindCommand.execute(vec!["hello"]).is_ok());

        assert!(FindCommand.execute(vec![]).is_err());
    }

    #[test]
    fn can_map_sort() {
        assert!(SortCommand.execute(vec!["ASC"]).is_ok());
        assert!(SortCommand.execute(vec!["DESC"]).is_ok());

        assert!(SortCommand.execute(vec!["UNKNOWN"]).is_err());
    }

    #[test]
    fn can_map_take() {
        assert!(TakeCommand.execute(vec!["1"]).is_ok());
        assert!(TakeCommand.execute(vec!["NaN"]).is_err());
    }
}
