use tonic::{transport::Server, Request, Response, Status, Streaming};

use crate::{proto::{
    logging_stream_server::{LoggingStream, LoggingStreamServer},
    Empty, LogRequest,
}, log_service};

#[derive(Default)]
pub struct LoggingStreamImpl {}

#[tonic::async_trait]
impl LoggingStream for LoggingStreamImpl {
    async fn log(
        &self,
        request: Request<Streaming<LogRequest>>,
    ) -> Result<Response<Empty>, Status> {
        let mut stream = request.into_inner();
        while let Some(log) = stream.message().await? {
            // got log message from client
            if let Err(err) = log_service::add_log(log).await {
                error!("unable to save log {}", err);
            };
        }
        Ok(Response::new(Empty {}))
    }
}

pub async fn start_log_gathering_server() {
    let uri =
        std::env::var("STORAGE_SERVER_URI").expect("env variable STORAGE_SERVER_URI was not found");
    let server = Server::builder()
        .add_service(LoggingStreamServer::new(LoggingStreamImpl::default()))
        .serve(uri.parse().unwrap())
        .await;
    if let Err(err) = server {
        panic!("Error in log gathering server: {}", err);
    }
}
