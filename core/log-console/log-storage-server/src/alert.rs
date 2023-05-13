use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{log_service, model::MonitorQuery, monitor_service};

/// all messages that can be sent to the alerting system
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AlertMessage {
    Create(MonitorQuery),
    Drop(MonitorQuery),
}

type Alerts = Arc<Mutex<HashSet<AlertMessage>>>;

#[derive(Clone)]
pub struct AlertHandler {
    alerts: Alerts,
    tx: crossbeam_channel::Sender<AlertMessage>,
    rx: crossbeam_channel::Receiver<AlertMessage>,
}

impl AlertHandler {
    fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(10);
        Self {
            alerts: Arc::default(),
            tx,
            rx,
        }
    }

    pub fn notify(&self, msg: AlertMessage) -> anyhow::Result<()> {
        self.alerts.lock().unwrap().insert(msg.clone());
        self.tx.send(msg)?;
        Ok(())
    }
}

/// starts the alert monitoring and returns a sender which can be used to comunicate with the
/// monitoring system
pub async fn start_alerts() -> anyhow::Result<AlertHandler> {
    let handler = AlertHandler::new();
    let create_handler = handler.clone();
    thread::spawn(move || {
        for msg in create_handler.rx.recv().iter() {
            if let AlertMessage::Create(monitor_query) = msg {
                let handler = create_handler.clone();
                let query = monitor_query.clone();
                thread::spawn(move || monitor(handler, query));
            }
        }
    });

    for query in monitor_service::get_all_monitor_queries().await? {
        let monitor_query = MonitorQuery {
            query: query.get_str("query")?.to_string(),
            interval: query.get_str("interval")?.to_string(),
        };
        let handler = handler.clone();
        thread::spawn(move || monitor(handler, monitor_query));
    }
    Ok(handler)
}

/// monitors a single query.
/// sleeps for the specified interval and then executes the specified query if the process has not been
/// dropped.
async fn monitor(handler: AlertHandler, monitor_query: MonitorQuery) {
    let duration = match get_duration_from_interval(&monitor_query.interval) {
        Ok(duration) => duration,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    loop {
        if handler
            .alerts
            .lock()
            .unwrap()
            .contains(&AlertMessage::Drop(monitor_query.clone()))
        {
            info!("Stopping monitoring for query {} because the query was deleted", monitor_query.query);
            return;
        }
        match log_service::run_query(&monitor_query.query).await {
            Ok(results) => {
                if !results.is_empty() {
                    info!("Sending alert, because query {} found results", monitor_query.query);
                    println!("ALERT: {:?}", results);
                }
            }
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        thread::sleep(duration);
    }
}

fn get_duration_from_interval(interval: &str) -> anyhow::Result<Duration> {
    // minutes
    if interval.ends_with('m') {
        let duration = interval[0..interval.len() - 1].parse::<u64>()?;
        Ok(Duration::from_millis(duration * 60_000))
    // milliseconds
    } else if interval.ends_with("ms") {
        let duration = interval[0..interval.len() - 2].parse::<u64>()?;
        Ok(Duration::from_millis(duration))
    // seconds
    } else if interval.ends_with('s') {
        let duration = interval[0..interval.len() - 1].parse::<u64>()?;
        Ok(Duration::from_millis(duration * 1000))
    // hours
    } else if interval.ends_with('h') {
        let duration = interval[0..interval.len() - 1].parse::<u64>()?;
        Ok(Duration::from_millis(duration * 3_600_000))
    // days
    } else if interval.ends_with('d') {
        let duration = interval[0..interval.len() - 1].parse::<u64>()?;
        Ok(Duration::from_millis(duration * 3_600_000 * 24))
    } else {
        Err(anyhow::anyhow!(
            "Invalid interval provided {} to interval",
            interval
        ))
    }
}
