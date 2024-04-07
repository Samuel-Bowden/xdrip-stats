use crate::status::Status;
use chrono::{DateTime, Local};
use std::fmt::Display;

pub struct Tooltip<'a> {
    time: DateTime<Local>,
    status: &'a Status,
}

impl<'a> Tooltip<'a> {
    pub fn new(time: DateTime<Local>, status: &'a Status) -> Self {
        Self { time, status }
    }
}

impl<'a> Display for Tooltip<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = self.time.format("%H:%M:%S");
        if let Status::Error(msg) = &self.status {
            return write!(f, "Read: {}\nError: {}", time, msg);
        }
        write!(f, "Read: {}", time)
    }
}
