use std::str::FromStr;

use anyhow::anyhow;
use mongodb::{bson::{doc, oid::ObjectId, Document}, Database};
use rocket::futures::TryStreamExt;

use crate::{connection, model::MonitorQuery};

pub async fn create_monitor_query(monitor_query: &MonitorQuery) -> mongodb::error::Result<()> {
    connection::get_connection()
        .await?
        .collection::<MonitorQuery>("monitor_query")
        .insert_one(monitor_query, None)
        .await?;
    Ok(())
}

pub async fn delete_monitor_query(db: &Database, id: &str) -> mongodb::error::Result<()> {
    let id = match ObjectId::from_str(id) {
        Ok(id) => id,
        Err(_) => {
            return Err(mongodb::error::Error::custom(anyhow!(
                "Invalid id provided"
            )))
        }
    };
    db
        .collection::<MonitorQuery>("monitor_query")
        .delete_one(doc! { "_id": id }, None)
        .await?;
    Ok(())
}

pub async fn get_monitor_query_by_id(db: &Database, id: &str) -> mongodb::error::Result<Option<MonitorQuery>> {
    let id = match ObjectId::from_str(id) {
        Ok(id) => id,
        Err(_) => {
            return Err(mongodb::error::Error::custom(anyhow!(
                "Invalid id provided"
            )))
        }
    };
    let monitor_query = db
        .collection::<MonitorQuery>("monitor_query")
        .find_one(doc! { "_id": id }, None)
        .await?;
    Ok(monitor_query)
}

pub async fn get_all_monitor_queries() -> mongodb::error::Result<Vec<Document>> {
    connection::get_connection()
        .await?
        .collection("monitor_query")
        .find(None, None)
        .await?
        .try_collect::<Vec<Document>>()
        .await
}
