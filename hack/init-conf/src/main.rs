use std::{fs, thread};

use pub_sub::{PubSub, Subscription};
use serde::Deserialize;

// read directory names
// start process for each of them
// in process:
//  - process config.yml file
//  - potentially wait for other processes to complete
//  - execute the docker compose in the directory
//  - notify that this process is done
fn main() {
    // Read configurations and open a thread for each of them
    let config_names = fs::read_dir("conf")
        .expect("expected a conf directory but it was not found")
        .filter(|dir| match dir {
            Ok(dir) => dir.metadata().unwrap().is_dir(),
            Err(_) => false,
        })
        .map(|conf_dir| {
            conf_dir
                .unwrap()
                .file_name()
                .into_string()
                .expect("Unable to parse OSString to String")
        })
        .collect::<Vec<String>>();

    let channel = pub_sub::PubSub::<&str>::new();
    let mut config_processes = vec![];
    for name in config_names {
        let process = ConfigProcess::new(name, channel.clone(), channel.subscribe());
        config_processes.push(thread::spawn(move || process.process_config()));
    }

    for proceess in config_processes {
        proceess.join().unwrap();
    }
}

struct ConfigProcess<'r> {
    name: String,
    channel: PubSub<&'r str>,
    rx: Subscription<&'r str>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    depends_on: Option<Vec<String>>,
    after_start: Option<String>,
}

impl<'r> ConfigProcess<'r> {
    fn new(name: String, channel: PubSub<&'r str>, rx: Subscription<&'r str>) -> Self {
        Self { name, channel, rx }
    }

    pub fn process_config(&self) {
        let config = self.load_config();
        if let Some(config) = &config {
            self.pre_deploy(config);
        }
        Self::deploy();
        if let Some(config) = &config {
            self.post_deploy(config);
        }
        println!("completed config for {}", self.name);
    }

    fn load_config(&self) -> Option<Config> {
        match fs::read_to_string(format!("conf/{}/config.yml", self.name)) {
            Ok(config) => serde_yaml::from_str(&config).ok(),
            Err(err) => {
                eprintln!("Error in config file for config {}: {}", self.name, err);
                None
            }
        }
    }

    fn pre_deploy(&self, config: &Config) {
        todo!()
    }

    fn post_deploy(&self, config: &Config) {
        todo!()
    }

    fn deploy() {
        todo!()
    }
}
