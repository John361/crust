use chrono::{DateTime, Local, Timelike};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub schedule: Schedule,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub minute: u32,
    pub hour: u32,
}

impl Task {
    pub fn execute(&self) {
        if self.can_be_activated() && self.schedule.is_time() {
            println!("{}", self.command);
        }
    }

    fn can_be_activated(&self) -> bool {
        !self.schedule.is_time_pasted()
    }
}

impl Schedule {
    pub fn is_time(&self) -> bool {
        let now: DateTime<Local> = Local::now();
        now.hour() == self.hour && now.minute() == self.minute
    }

    pub fn is_time_pasted(&self) -> bool {
        let now: DateTime<Local> = Local::now();
        now.hour() > self.hour && now.minute() > self.minute
    }
}
