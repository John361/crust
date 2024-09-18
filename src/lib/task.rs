use std::process::Command;

use chrono::{DateTime, Local, Timelike};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Task {
    pub command: String,
    pub schedule: Schedule,
}

impl Task {
    pub fn execute(&self) -> anyhow::Result<()> {
        if self.schedule.is_time() {
            Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .spawn()?;
        }

        Ok(())
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
