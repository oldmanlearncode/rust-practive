use std::collections::HashMap;

use crate::validate_title;

pub enum JobState {
    Queued,
    Printed,
    Cancelled,
}
pub enum QueueError {
    InvalidTitle,
    InvalidPages,
    Full,
    Missing,
    NotQueued,
}

pub struct Job {
    title: String,
    pages: u32,
    state: JobState,
}
impl Job {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn pages(&self) -> u32 {
        self.pages
    }
    pub fn state(&self) -> &JobState {
        &self.state
    }
}

pub struct Spool {
    capacity: usize,
    jobs: Vec<Job>,
}
impl Spool {
    pub fn new(capacity: usize) -> Self {
        Spool {
            capacity,
            jobs: vec![],
        }
    }
    pub fn len(&self) -> usize {
        self.jobs.len()
    }
    pub fn job(&self, index: usize) -> Option<&Job> {
        self.jobs.get(index)
    }
    pub fn submit(&mut self, title: String, pages: u32) -> Result<usize, QueueError> {
        if validate_title(&title).is_err() {
            return Err(QueueError::InvalidTitle);
        }
        if !(1..=100).contains(&pages) {
            return Err(QueueError::InvalidPages);
        }
        if self.jobs.len().eq(&self.capacity) {
            return Err(QueueError::Full);
        }

        self.jobs.push(Job {
            title,
            pages,
            state: JobState::Queued,
        });
        Ok(self.jobs.len() - 1)
    }

    pub fn cancel(&mut self, index: usize) -> Result<(), QueueError> {
        match self.jobs.get_mut(index) {
            None => Err(QueueError::Missing),
            Some(job) => match job.state() {
                JobState::Printed => Err(QueueError::NotQueued),
                JobState::Cancelled => Err(QueueError::NotQueued),
                JobState::Queued => {
                    job.state = JobState::Cancelled;
                    Ok(())
                }
            },
        }
    }

    pub fn print_next(&mut self) -> Option<usize> {
        let mut index: usize = 0;
        for job in &mut self.jobs {
            match job.state() {
                JobState::Queued => {
                    job.state = JobState::Printed;
                    return Some(index);
                }
                _ => index += 1,
            }
        }
        None
    }
    pub fn pending_pages(&self) -> u32 {
        todo!("Tổng pages của Queued")
    }
    pub fn queued_counts(&self) -> HashMap<String, usize> {
        todo!("Map mới đếm title của Queued bằng entry/or_insert")
    }
}
