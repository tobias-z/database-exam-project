use anyhow::anyhow;
use mongodb::bson::{Document, doc, DateTime};

pub trait QueryCommand {
    fn execute(&self, args: Vec<&str>) -> anyhow::Result<Document>;
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
