use crate::{Grade, PracticeError};
use std::collections::HashMap;

pub struct Question {
    label: String,
    left: u32,
    right: u32,
    answer: Option<u32>,
}
impl Question {
    pub fn label(&self) -> &str { todo!() }
    pub fn factors(&self) -> (u32, u32) { todo!() }
    pub fn answer(&self) -> Option<u32> { todo!() }
}

pub struct PracticeSession {
    questions: Vec<Question>,
}
impl PracticeSession {
    pub fn new() -> Self { todo!() }
    pub fn len(&self) -> usize { todo!() }
    pub fn add(&mut self, label: String, left: u32, right: u32) -> Result<usize, PracticeError> { todo!() }
    pub fn question(&self, index: usize) -> Option<&Question> { todo!() }
    pub fn answer(&mut self, index: usize, value: u32) -> Result<(), PracticeError> { todo!() }
    pub fn result(&self, index: usize) -> Result<Grade, PracticeError> { todo!() }
    pub fn retry_labels(&self) -> Vec<String> { todo!() }
    pub fn counts(&self) -> HashMap<String, usize> { todo!() }
}
