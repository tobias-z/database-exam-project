use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub container_id: String,
    pub container_name: String,
    pub message: String,
    pub date: mongodb::bson::DateTime,
    /// Here the assumption is that it is faster to do comparisons on integers than strings.
    /// Integers also fill less in memory
    pub level: Option<u8>,
}
