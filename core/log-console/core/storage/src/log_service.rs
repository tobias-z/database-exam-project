use crate::{connection, model::Log, proto::LogRequest};

pub async fn add_log(request: LogRequest) -> mongodb::error::Result<()> {
    if is_blacklisted(&request.container_name) {
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

fn is_blacklisted(container_name: &str) -> bool {
    let blacklist = std::env::var("CONTAINER_BLACKLIST")
        .expect("env variable CONTAINER_BLACKLIST was not found");
    blacklist
        .split(',')
        .any(|item| container_name.contains(item))
}

fn get_log_level(message: &str) -> Option<u8> {
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
        _ => None
    }
}
