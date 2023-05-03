use std::collections::HashSet;

use mongodb::bson::Document;
use rocket::futures::TryStreamExt;

use crate::{connection, model::Log, proto::LogRequest, query_lang};

pub struct LogService {
    blacklist: Blacklist,
}

impl Default for LogService {
    fn default() -> Self {
        Self {
            blacklist: Blacklist::new(),
        }
    }
}

impl LogService {
    pub async fn add_log(&self, request: LogRequest) -> mongodb::error::Result<()> {
        if self.blacklist.is_blacklisted(&request.container_name) {
            return Ok(());
        }
        let log: Log = Log {
            container_name: request.container_name,
            container_id: request.container_id,
            level: get_log_level(&request.message),
            message: request.message,
            date: mongodb::bson::DateTime::parse_rfc3339_str(request.date)
                .expect("the date in the log request was an invalid bson datatime"),
        };
        let db = connection::get_connection().await?;
        let collection = db.collection::<Log>("logs");
        collection.insert_one(log, None).await?;
        Ok(())
    }
}

pub async fn run_query(query: &str) -> anyhow::Result<Vec<Document>> {
    let aggregation = query_lang::parse::into_aggregation(query)?;
    let documents = connection::get_connection()
        .await?
        .collection::<Log>("logs")
        .aggregate(aggregation, None)
        .await?
        .try_collect::<Vec<Document>>()
        .await?;
    Ok(documents)
}

pub fn get_log_level(message: &str) -> Option<u8> {
    let levels = ["FATAL", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];
    for level in levels {
        if message.contains(level) {
            return log_to_u8(level);
        }
    }
    None
}

fn log_to_u8(log: &str) -> Option<u8> {
    match log {
        "FATAL" => Some(1),
        "ERROR" => Some(2),
        "WARN" => Some(3),
        "INFO" => Some(4),
        "DEBUG" => Some(5),
        "TRACE" => Some(6),
        _ => None,
    }
}

struct Blacklist {
    blacklist: HashSet<String>,
}

impl Blacklist {
    fn new() -> Self {
        let blacklist = std::env::var("CONTAINER_BLACKLIST")
            .expect("env variable CONTAINER_BLACKLIST was not found")
            .split(',')
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();
        Self { blacklist }
    }
    fn is_blacklisted(&self, container_name: &str) -> bool {
        self.blacklist.contains(container_name)
    }
}
