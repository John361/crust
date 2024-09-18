use chrono::{DateTime, Local, Timelike};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Task {
    pub schedule: Schedule,
    pub command: String,
}

impl Task {
    pub fn execute(&self) {
        if self.schedule.is_time() {
            println!("{}", self.command);
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schedule {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Schedule {
    pub fn is_time(&self) -> bool {
        let now: DateTime<Local> = Local::now();
        now.hour() == self.hour && now.minute() == self.minute && now.second() == self.second
    }
}
