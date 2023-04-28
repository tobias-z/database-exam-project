use anyhow::anyhow;
use mongodb::bson::{doc, DateTime, Document};

use crate::log_service;

pub trait QueryCommand {
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()>;
}

pub struct ContainerCommand;

impl QueryCommand for ContainerCommand {
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
        let Some(container_name) = args.first() else {
            return Err(anyhow!("No container name provided in container query command"));
        };
        push(doc! {
            "$match": {
                "container_name": container_name
            }
        });
        Ok(())
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
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
        let Some(time_arg) = args.first() else {
            return Err(anyhow!("No time arg provided in log level query command"));
        };
        push(doc! {
            "$match": {
                "date": {
                    "$gte": ["$date", self.get_datetime_from_range(time_arg)?]
                }
            }
        });
        Ok(())
    }
}

pub struct LogLevelCommand;

impl QueryCommand for LogLevelCommand {
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
        let Some(log_level) = args.first() else {
            return Err(anyhow!("No log level provided in log_level query command"));
        };
        // map the given log level to the stored number for that log level
        match log_service::get_log_level(log_level) {
            Some(log_level) => Ok(push(doc! {
                "$match": {
                    "level": log_level as i32,
                }
            })),
            None => Err(anyhow!("invalid log level provided {}", log_level)),
        }
    }
}

pub struct FindCommand;

impl QueryCommand for FindCommand {
    /// This makes use of the a text index in mongodb
    /// https://www.mongodb.com/docs/manual/core/index-text/
    /// We index the message field as a text index.
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
        if args.is_empty() {
            return Err(anyhow!("No search term provided to the find command"));
        }
        push(doc! {
            "$text": {
                "$search": args.join(" "),
            },
        });
        // sort the results by their score
        push(doc! {
            "$sort": {
                "score": {
                    "$meta": "textScore"
                }
            }
        });
        Ok(())
    }
}

pub struct SortCommand;

impl QueryCommand for SortCommand {
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
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
        push(doc! {
            "$sort": {
                "date": direction
            }
        });
        Ok(())
    }
}

pub struct TakeCommand;

impl QueryCommand for TakeCommand {
    fn execute(&self, args: Vec<&str>, push: &mut impl FnMut(Document)) -> anyhow::Result<()> {
        let Some(amount) = args.first() else {
            return Err(anyhow!("No amount provided in take query command"));
        };
        let Ok(amount) = amount.parse::<i32>() else {
            return Err(anyhow!("Expected the second parmater to the take query command, but got {}", amount));
        };
        push(doc! {
            "$limit": amount
        });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn execute_command(query_command: impl QueryCommand, args: Vec<&str>) -> anyhow::Result<()> {
        let mut arr = vec![];
        query_command.execute(args, &mut |doc| arr.push(doc))
    }

    #[test]
    fn can_map_container() {
        assert!(execute_command(ContainerCommand, vec!["something"]).is_ok());
        assert!(execute_command(ContainerCommand, vec![]).is_err());
    }

    #[test]
    fn can_map_offset() {
        assert!(execute_command(OffsetCommand, vec!["10ms"]).is_ok());
        assert!(execute_command(OffsetCommand, vec!["10m"]).is_ok());
        assert!(execute_command(OffsetCommand, vec!["10s"]).is_ok());
        assert!(execute_command(OffsetCommand, vec!["10h"]).is_ok());
        assert!(execute_command(OffsetCommand, vec!["10d"]).is_ok());

        assert!(execute_command(OffsetCommand, vec!["10j"]).is_err());
    }

    #[test]
    fn can_map_log_level() {
        assert!(execute_command(LogLevelCommand, vec!["INFO"]).is_ok());
        assert!(execute_command(LogLevelCommand, vec!["ERROR"]).is_ok());
        assert!(execute_command(LogLevelCommand, vec!["WARN"]).is_ok());
        assert!(execute_command(LogLevelCommand, vec!["TRACE"]).is_ok());
        assert!(execute_command(LogLevelCommand, vec!["DEBUG"]).is_ok());
        assert!(execute_command(LogLevelCommand, vec!["FATAL"]).is_ok());

        assert!(execute_command(LogLevelCommand, vec!["UNKNOWN"]).is_err());
    }

    #[test]
    fn can_map_find() {
        assert!(execute_command(FindCommand, vec!["hello", "world"]).is_ok());
        assert!(execute_command(FindCommand, vec!["hello"]).is_ok());

        assert!(execute_command(FindCommand, vec![]).is_err());
    }

    #[test]
    fn can_map_sort() {
        assert!(execute_command(SortCommand, vec!["ASC"]).is_ok());
        assert!(execute_command(SortCommand, vec!["DESC"]).is_ok());

        assert!(execute_command(SortCommand, vec!["UNKNOWN"]).is_err());
    }

    #[test]
    fn can_map_take() {
        assert!(execute_command(TakeCommand, vec!["1"]).is_ok());
        assert!(execute_command(TakeCommand, vec!["NaN"]).is_err());
    }
}
