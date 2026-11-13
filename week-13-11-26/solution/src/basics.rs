use std::collections::HashMap;

pub enum Event {
    Healthy,
    Failed,
}

pub enum LogError {
    BadFormat,
    BlankService,
    BadStatus,
    Full,
}

pub fn split_record(line: &str) -> Result<(&str, &str), LogError> {
    todo!("Bài 1")
}

pub fn longest_failure_run(events: &[Event]) -> usize {
    todo!("Bài 2")
}

pub fn index_labels(labels: Vec<String>) -> HashMap<String, Vec<usize>> {
    todo!("Bài 3")
}
