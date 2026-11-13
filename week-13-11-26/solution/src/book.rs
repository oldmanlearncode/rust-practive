use crate::{Event, LogError};
use std::collections::HashMap;

pub struct LogBook {
    services: HashMap<String, Vec<Event>>,
}

impl LogBook {
    pub fn new() -> Self { todo!() }
    pub fn service_count(&self) -> usize { todo!() }
    pub fn events(&self, service: &str) -> Option<&[Event]> { todo!() }
    pub fn record(&mut self, line: &str) -> Result<usize, LogError> { todo!() }
    pub fn longest_run(&self, service: &str) -> Option<usize> { todo!() }
}
