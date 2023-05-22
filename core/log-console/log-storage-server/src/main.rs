pub mod alert;
pub mod connection;
pub mod gather;
pub mod log_service;
pub mod model;
pub mod monitor_service;
pub mod query_lang;
pub mod rest;
pub mod auth;

#[macro_use]
extern crate log;

#[macro_use]
extern crate rocket;

use gather::start_log_gathering_server;

pub mod proto {
    tonic::include_proto!("logconsole.proto");
    tonic::include_proto!("dk.groupa.proto");
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    tokio::spawn(start_log_gathering_server());
    let alerter = alert::start_alerts().await.unwrap();
    rest::start_rest_server(alerter).await?;
    Ok(())
}
