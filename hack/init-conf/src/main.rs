use std::{fs, thread, time::Duration};

use pub_sub::{PubSub, Subscription};

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

impl<'r> ConfigProcess<'r> {
    fn new(name: String, channel: PubSub<&'r str>, rx: Subscription<&'r str>) -> Self {
        Self { name, channel, rx }
    }

    fn process_config(&self) {
        println!("completed config for {}", self.name);
    }
}
