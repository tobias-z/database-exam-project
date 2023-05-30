/// logs come in as a single line, and some logs are stacktraces that should be a
/// single multiline log.
/// https://grafana.com/docs/loki/latest/clients/promtail/stages/multiline/ explains
/// this problem perfectly. What we need is to ensure that the complete stack trace
/// is included in the log.
///
/// 1. Have specific checks for specific kinds of logs. Logstash, Rocket, etc.
/// 2. If a match is found on the last line of the log (i think), wait for a specific amount of time before running the
///    log gathering again.
/// 3. Join all the lines that are specific to a single log together
use crate::proto::LogRequest;
use lazy_static::lazy_static;

pub trait MultilineMatcher: Send + Sync {
    fn is_match(&self, log_message: &str) -> bool;
}

pub struct LogstashMatcher<'a> {
    package_regex: regex::Regex,
    caused_by: &'a str,
}

impl Default for LogstashMatcher<'_> {
    fn default() -> Self {
        Self {
            package_regex: regex::Regex::new(r"^(?:\w+|\w+\.\w+)+: .*$").unwrap(),
            caused_by: "Caused by",
        }
    }
}

impl MultilineMatcher for LogstashMatcher<'_> {
    fn is_match(&self, log_message: &str) -> bool {
        log_message.starts_with('\t')
            // some stack trace logs proceed on the next line with 'Caused by ...'
            || log_message.starts_with(self.caused_by)
            // starts e.g. 'org.example.hello: '
            || self.package_regex.is_match(log_message)
    }
}

lazy_static! {
    static ref MATCHERS: [Box<dyn MultilineMatcher>; 1] = [Box::<LogstashMatcher>::default()];
}

/// If any of the matchers finds that this is a multiline message, this returns true
pub fn is_multiline(log_message: &str) -> bool {
    MATCHERS.iter().any(|matcher| matcher.is_match(log_message))
}

/// maps multiline logs together into a single log
/// a multiline log is said to be complete when the next line does not match any multiline check
pub fn join_multiline_logs_together(logs: Vec<LogRequest>) -> Vec<LogRequest> {
    let mut final_logs = Vec::new();
    // used to keep appending the logs into
    let mut curr_log = LogRequest::default();

    for (i, log) in logs.iter().enumerate() {
        if curr_log.message.is_empty() {
            curr_log.container_name = log.container_name.clone();
            curr_log.container_id = log.container_id.clone();
            curr_log.date = log.date.clone();
        }
        curr_log.message.push_str(&log.message);

        if let Some(next) = logs.get(i + 1) {
            if is_multiline(&next.message) {
                // the next log is a multiline log, so it should be appended to the current one
                curr_log.message.push('\n');
                continue;
            }
        }

        final_logs.push(curr_log.clone());
        curr_log.message = String::new();
    }
    final_logs
}
