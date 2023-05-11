use std::{thread, time::Duration};

// read directory names
// start process for each of them
// in process:
//  - process config.yml file
//  - potentially wait for other processes to complete
//  - execute the docker compose in the directory
//  - notify that this process is done
fn main() {
    // Read configurations and open a thread for each of them
    let (tx, rx) = crossbeam_channel::bounded::<&str>(10);
    let mut config_processes = vec![];
    for i in 0..10 {
        let tx = tx.clone();
        let rx = rx.clone();
        config_processes.push(thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            println!("{}: {}", i, rx.recv().unwrap());
        }));
    }
    for proceess in config_processes {
        proceess.join();
    }
    tx.send("hello world").unwrap();
}
