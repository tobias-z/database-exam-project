use std::str::FromStr;

use anyhow::anyhow;
use mongodb::bson::{doc, oid::ObjectId, Document};
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

pub async fn delete_monitor_query(id: String) -> mongodb::error::Result<()> {
    let id = match ObjectId::from_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err(mongodb::error::Error::custom(anyhow!(
                "Invalid id provided"
            )))
        }
    };
    connection::get_connection()
        .await?
        .collection::<MonitorQuery>("monitor_query")
        .delete_one(doc! { "_id": id }, None)
        .await?;
    Ok(())
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
