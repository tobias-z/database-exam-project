use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Seek},
    path::PathBuf,
    time::Duration,
};

use async_recursion::async_recursion;
use notify::{ErrorKind, EventKind};
use serde::Deserialize;
use tonic::transport::Channel;

use crate::{
    multiline,
    proto::{logging_stream_client::LoggingStreamClient, LogRequest},
    util, FsListener,
};

pub struct LogListener {
    /// The container id maps to our current position in the file.
    /// Im not sure if this will cause async problems. I don't think so as all calls to on_event is
    /// awaited
    active_log_files: HashMap<String, u64>,

    /// Channel sender used to notify about new messages that should be sent
    client: LoggingStreamClient<Channel>,

    // the location of where our containers are stored. This is kept on the struct for caching
    // purposes, as we don't want to read from the env on every log
    containers_location: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ConfigV2 {
    name: String,
}

impl LogListener {
    pub fn new(client: LoggingStreamClient<Channel>, containers_location: String) -> Self {
        Self {
            active_log_files: HashMap::default(),
            client,
            containers_location,
        }
    }

    fn get_config(&self, container_id: &str) -> anyhow::Result<ConfigV2> {
        let Ok(config) = fs::read_to_string(format!(
            "{}/{}/config.v2.json",
            self.containers_location, container_id
        )) else {
            return Err(anyhow::anyhow!("{}/{}/config.v2.json was not found", self.containers_location, container_id));
        };
        let mut config =
            serde_json::from_str::<ConfigV2>(&config).expect("name not found in the config file");
        config.name = config.name.replace(['\"', '/'], "");
        Ok(config)
    }

    #[async_recursion]
    async fn modify(
        &mut self,
        container_id: String,
        path: &PathBuf,
        last_pos: Option<usize>,
    ) -> anyhow::Result<()> {
        let config = self.get_config(&container_id)?;
        let position = self
            .active_log_files
            .entry(container_id.clone())
            .or_insert(0);
        let mut file = File::open(path).unwrap();
        // move to the stored position of our log file
        file.seek(std::io::SeekFrom::Start(*position)).unwrap();
        // update our current position
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        // we might be notified about a change containing multiple new lines where each
        // line is a new log
        let logs: Vec<LogRequest> = buf
            .lines()
            .map(serde_json::from_str::<DockerLog>)
            .filter(Result::is_ok)
            .map(|docker_log| {
                let docker_log = docker_log.unwrap();
                LogRequest {
                    message: util::trim_newline(docker_log.log),
                    date: docker_log.time,
                    container_id: container_id.clone(),
                    container_name: config.name.clone(),
                }
            })
            .filter(|log| !log.message.is_empty())
            .collect();

        if !logs.is_empty() {
            // Last log position is used to keep track of the current log in question
            //
            // Example:
            //  1. A multiline log comes in. In the beginning we have no 'last_pos', so we use the end
            //  of the logs as the bottom.
            //  2. We find out that this line is a multiline, so we wait for the whole log to come
            //     in.
            //  3. Call the modify function again after the delay and provide it the position that
            //     was previously used, so that we don't skip to the end of the log file again.
            let last_log_pos = logs.len() - 2;
            if let Some(log) = &logs.get(last_pos.unwrap_or(last_log_pos)) {
                if multiline::is_multiline(&log.message) {
                    // wait for the rest of the multilined log to come in
                    std::thread::sleep(Duration::from_secs(3));
                    return self.modify(container_id, path, Some(last_log_pos)).await;
                }
            }

            // set the new current position to the end of the file.
            *position = file.metadata().unwrap().len();

            let logs = multiline::join_multiline_logs_together(logs);
            if let Err(status) = self.client.log(futures::stream::iter(logs)).await {
                error!("Unable to send logs. Status code: {}", status);
            };
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct DockerLog {
    log: String,
    time: String,
}

#[async_trait::async_trait]
impl FsListener for LogListener {
    /// This listener acts on the events: Create, Modify and Remove.
    /// When a new container is created, we add it to the log files we are tracking
    /// When a container is removed, we remove it from our log files
    /// When a log file is edited, we find the newly inserted logs, and send them to the storage
    /// server.
    async fn on_event(&mut self, event: notify::Event) -> anyhow::Result<()> {
        let Some(path) = event.paths.first() else {
            return Ok(());
        };
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !file_name.ends_with("-json.log") {
            // not a log file
            return Ok(());
        }
        let container_id = file_name.replace("-json.log", "").trim().to_string();
        match event.kind {
            EventKind::Create(_) => {
                let config = self.get_config(&container_id)?;
                info!(
                    "New container started with id {} and name: {}",
                    container_id, config.name
                );
                self.active_log_files.insert(container_id, 0);
            }
            EventKind::Modify(_) => {
                self.modify(container_id, path, None).await;
            }
            EventKind::Remove(_) => {
                info!("Container with id {} removed", container_id);
                self.active_log_files.remove(&container_id);
            }
            _ => {
                // ignoring the rest as we only care about creating deleting and updating
            }
        }
        Ok(())
    }

    fn on_error(&self, error: notify::Error) {
        match error.kind {
            ErrorKind::Generic(err) => error!(
                "Generic error when listening on paths {:?}, error: {}",
                error.paths, err
            ),
            ErrorKind::Io(err) => error!(
                "IO error when listening on paths {:?}, error: {}",
                error.paths, err
            ),
            ErrorKind::PathNotFound => {
                error!("Path not found when listening on paths {:?}", error.paths)
            }
            ErrorKind::WatchNotFound => {
                error!("Watch not found when listening on paths {:?}", error.paths)
            }
            ErrorKind::InvalidConfig(config) => error!(
                "Invalid config found listening on paths {:?}, config: {:?}",
                error.paths, config
            ),
            ErrorKind::MaxFilesWatch => error!("Max files reached. No more files can be watched"),
        }
    }
}
