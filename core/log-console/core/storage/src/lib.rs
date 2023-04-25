pub mod connection;
pub mod gather;
pub mod log_service;
pub mod model;
pub mod query_lang;

#[macro_use]
extern crate log;

use std::io;

use gather::start_log_gathering_server;

pub mod proto {
    tonic::include_proto!("logconsole.proto");
}

#[tokio::main]
pub async fn run() -> io::Result<()> {
    start_log_gathering_server().await;
    Ok(())
}
