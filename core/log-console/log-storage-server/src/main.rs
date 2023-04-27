pub mod connection;
pub mod gather;
pub mod log_service;
pub mod model;
pub mod query_lang;
mod rest;

#[macro_use]
extern crate log;

#[macro_use]
extern crate rocket;

use gather::start_log_gathering_server;

pub mod proto {
    tonic::include_proto!("logconsole.proto");
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    tokio::spawn(async move {
        start_log_gathering_server().await;
    });
    rest::run_rest_server().await?;
    Ok(())
}
