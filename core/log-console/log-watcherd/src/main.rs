pub mod log_listener;
pub mod multiline;
mod util;

#[macro_use]
extern crate log;

use std::path::Path;

use log_listener::LogListener;
use notify::{Config, Event, RecursiveMode, Watcher};
use proto::logging_stream_client::LoggingStreamClient;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("logconsole.proto");
}

#[async_trait::async_trait]
trait FsListener: Send {
    async fn on_event(&mut self, event: Event) -> anyhow::Result<()>;
    fn on_error(&self, error: notify::Error);
}

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let containers_location = std::env::var("CONTAINERS_LOCATION")
        .expect("env variable CONTAINERS_LOCATION was not found");
    listen(
        containers_location.clone(),
        Box::new(LogListener::new(get_client().await, containers_location)),
    )
    .await;
}

async fn get_client() -> LoggingStreamClient<Channel> {
    let uri =
        std::env::var("STORAGE_SERVER_URI").expect("env variable STORAGE_SERVER_URI was not found");
    LoggingStreamClient::connect(format!("http://{}", uri))
        .await
        .expect("unable to connect to gathering server with grpc")
}

async fn listen(container_location: String, mut listener: Box<dyn FsListener>) {
    let (tx, rx) = crossbeam_channel::bounded(100);
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher
        .watch(Path::new(&container_location), RecursiveMode::Recursive)
        .unwrap();

    loop {
        if let Ok(res) = rx.recv() {
            match res {
                Ok(event) => {
                    // We could greatly improve performance by implementing this stream processing
                    // with concurrent processing. However this would prob be a good amount of
                    // work, because we cant simply just put a lock on the hashmap, we would need
                    // to make it a bit more elegant, possibly having a lock in each file?
                    if let Err(e) = listener.on_event(event).await {
                        error!("{}", e);
                    };
                }
                Err(err) => listener.on_error(err),
            }
        }
    }
}
