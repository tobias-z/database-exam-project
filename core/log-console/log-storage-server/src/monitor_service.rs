use std::str::FromStr;

use anyhow::anyhow;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Database,
};
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

pub async fn delete_and_get_monitor_query(id: &str) -> anyhow::Result<MonitorQuery> {
    let client = connection::get_client()
        .await
        .expect("unable to connect to mongodb");
    let db = client.database("logs");
    let mut session = client.start_session(None).await.unwrap();
    session.start_transaction(None).await.unwrap();
    let Ok(Some(monitor_query)) = get_monitor_query_by_id(&db, id).await else {
        info!("No MonitorQuery found with id: {}", &id);
        return Err(anyhow!("No monitor query found with id {}", &id));
    };
    if let Err(e) = delete_monitor_query(&db, id).await {
        error!("Unable to delete MonitorQuery with id: {}. {}", id, e);
        return Err(anyhow!(e.to_string()));
    };
    session.commit_transaction().await.unwrap();
    Ok(monitor_query)
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
    db.collection::<MonitorQuery>("monitor_query")
        .delete_one(doc! { "_id": id }, None)
        .await?;
    Ok(())
}

pub async fn get_monitor_query_by_id(
    db: &Database,
    id: &str,
) -> mongodb::error::Result<Option<MonitorQuery>> {
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
