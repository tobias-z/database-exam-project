use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time;

use crate::{log_service, model::MonitorQuery, monitor_service};

/// all messages that can be sent to the alerting system
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AlertMessage {
    Create(MonitorQuery),
    Drop(MonitorQuery),
}

type Alerts = Arc<Mutex<HashSet<AlertMessage>>>;

#[derive(Clone)]
pub struct Alerter {
    alerts: Alerts,
    tx: crossbeam_channel::Sender<AlertMessage>,
    rx: crossbeam_channel::Receiver<AlertMessage>,
}

impl Alerter {
    fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(10);
        Self {
            alerts: Arc::default(),
            tx,
            rx,
        }
    }

    pub fn alert(&self, msg: AlertMessage) -> anyhow::Result<()> {
        self.alerts.lock().unwrap().insert(msg.clone());
        self.tx.send(msg)?;
        Ok(())
    }
}

/// starts the alert monitoring and returns a sender which can be used to comunicate with the
/// monitoring system
pub async fn start_alerts() -> anyhow::Result<Alerter> {
    let alerter = Alerter::new();
    let create_alerter = alerter.clone();
    tokio::spawn(async move {
        for msg in create_alerter.rx.recv().iter() {
            if let AlertMessage::Create(monitor_query) = msg {
                let alerter = create_alerter.clone();
                let query = monitor_query.clone();
                tokio::spawn(async move {
                    info!("Started monitoring of query '{}' running on interval '{}'", query.query, query.interval);
                    monitor(alerter, query).await
                });
            }
        }
    });

    for query in monitor_service::get_all_monitor_queries().await? {
        let monitor_query = MonitorQuery {
            query: query.get_str("query")?.to_string(),
            interval: query.get_str("interval")?.to_string(),
        };
        let alerter = alerter.clone();
        tokio::spawn(async move {
            info!("Started monitoring of query '{}' running on interval '{}'", monitor_query.query, monitor_query.interval);
            monitor(alerter, monitor_query).await;
        });
    }
    Ok(alerter)
}

/// monitors a single query.
/// sleeps for the specified interval and then executes the specified query if the process has not been
/// dropped.
async fn monitor(alerter: Alerter, monitor_query: MonitorQuery) {
    let duration = match get_duration_from_interval(&monitor_query.interval) {
        Ok(duration) => duration,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    loop {
        let drop = AlertMessage::Drop(monitor_query.clone());
        if alerter
            .alerts
            .lock()
            .unwrap()
            .contains(&drop)
        {
            alerter.alerts.lock().unwrap().remove(&drop);
            info!(
                "Stopping monitoring for query '{}' because the query was deleted",
                monitor_query.query
            );
            return;
        }
        info!("Running monitoring query {}", monitor_query.query);
        match log_service::run_query(&monitor_query.query).await {
            Ok(results) => {
                if !results.is_empty() {
                    info!(
                        "Sending alert, because query '{}' found results",
                        monitor_query.query
                    );
                    println!("ALERT: {:?}", results);
                }
            }
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        time::sleep(duration).await;
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
