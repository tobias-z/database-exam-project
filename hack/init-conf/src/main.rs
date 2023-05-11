use std::{collections::HashSet, fs, path, thread};

use anyhow::anyhow;
use pub_sub::{PubSub, Subscription};
use run_script::run_script;
use serde::Deserialize;

#[derive(Clone)]
enum Notification {
    Success(String),
    Error,
}

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

    let channel = pub_sub::PubSub::<Notification>::new();
    let mut config_processes = vec![];
    for name in config_names {
        let process = ConfigProcess::new(name, channel.clone(), channel.subscribe());
        config_processes.push(thread::spawn(move || match process.process_config() {
            Ok(_) => Ok(()),
            Err(e) => {
                process.tx.send(Notification::Error)?;
                Err(e)
            }
        }));
    }

    for proceess in config_processes {
        match proceess.join().unwrap() {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "An error happened during the initialization of the config: {}",
                    e
                );
            }
        };
    }
}

struct ConfigProcess {
    name: String,
    tx: PubSub<Notification>,
    rx: Subscription<Notification>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    depends_on: Option<HashSet<String>>,
    before_start: Option<String>,
    after_start: Option<String>,
}

impl ConfigProcess {
    fn new(name: String, tx: PubSub<Notification>, rx: Subscription<Notification>) -> Self {
        Self { name, tx, rx }
    }

    pub fn process_config(&self) -> anyhow::Result<()> {
        let config = self.load_config();
        if let Some(config) = &config {
            self.pre_deploy(config)?;
        }
        self.deploy()?;
        if let Some(config) = &config {
            self.post_deploy(config)?;
        }
        println!("{}: Completed config", self.name);
        self.tx.send(Notification::Success(self.name.clone()))?;
        Ok(())
    }

    fn load_config(&self) -> Option<Config> {
        let config = fs::read_to_string(format!("conf/{}/config.yml", self.name)).ok()?;
        match serde_yaml::from_str(&config) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!("{}: Error in config file: {}", self.name, e);
                None
            }
        }
    }

    fn pre_deploy(&self, config: &Config) -> anyhow::Result<()> {
        println!("{}: Found config", self.name);
        if let Some(depends_on) = &config.depends_on {
            if !depends_on.is_empty() {
                self.wait_for_processes(depends_on)?;
                println!(
                    "{}: Done waiting for processes: {:?}",
                    self.name, depends_on
                );
            }
        }
        if let Some(before_start) = &config.before_start {
            println!(
                "{}: Running before-start script: {}",
                self.name, before_start
            );
            run_script(format!("{} before-start", self.name), before_start)?;
        }
        Ok(())
    }

    fn post_deploy(&self, config: &Config) -> anyhow::Result<()> {
        if let Some(after_start) = &config.after_start {
            println!("{}: Running after-start script: {}", self.name, after_start);
            run_script(format!("{} after-start", self.name), after_start)?;
        }
        Ok(())
    }

    fn deploy(&self) -> anyhow::Result<()> {
        let config_dir = format!("conf/{}", self.name);
        if path::Path::new(&format!("{}/docker-compose.yml", config_dir)).try_exists()? {
            println!("{}: Running docker-compose.yml", self.name);
            run_script(
                format!("{} docker-compose", self.name),
                &format!(
                    r#"
                cd {}
                docker compose up -d
            "#,
                    config_dir
                ),
            )?;
        };
        Ok(())
    }

    fn wait_for_processes(&self, depends_on: &HashSet<String>) -> anyhow::Result<()> {
        let mut completed = 0;
        println!("{}: Waiting for processes: {:?}", self.name, depends_on);
        while let Ok(process_done) = self.rx.recv() {
            match process_done {
                Notification::Success(process_done) => {
                    if depends_on.contains(&process_done) {
                        completed += 1;
                    }
                    if completed == depends_on.len() {
                        return Ok(());
                    }
                }
                Notification::Error => {
                    return Err(anyhow!(
                        "Stopping process for {} because of an error in an other process",
                        self.name
                    ));
                }
            }
        }
        Err(anyhow!("All senders were killed"))
    }
}

fn run_script(name: String, script: &str) -> anyhow::Result<()> {
    let (code, output, error) = run_script!(script).unwrap();
    println!("{}: exited with code: {}", name, code);
    if code == 0 {
        if script.contains("docker compose up") {
            // For some reason docker command output is thrown into the error output
            println!("{}: {}", name, error);
        } else {
            println!("{}: {}", name, output);
        }
        Ok(())
    } else {
        Err(anyhow!("{}: {}", name, error))
    }
}
