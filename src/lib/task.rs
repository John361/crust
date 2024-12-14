use std::io::Read;
use std::process::{Command, Stdio};

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
    pub fn execute(&self) -> anyhow::Result<TaskStatus> {
        if self.schedule.is_time() {
            let mut execution = Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .stderr(Stdio::piped())
                .spawn()?;

            let status = execution.wait()?;
            return if status.success() {
                Ok(TaskStatus::Success)
            } else {
                let mut stderr = String::new();
                if let Some(mut stderr_pipe) = execution.stderr.take() {
                    stderr_pipe.read_to_string(&mut stderr)?;
                }

                Ok(TaskStatus::Error(stderr))
            }
        }

        Ok(TaskStatus::NotReady)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schedule {
    hour: String,
    minute: String,
    second: String,
}

impl Schedule {
    pub fn is_time(&self) -> bool {
        let now: DateTime<Local> = Local::now();
        let hour = self.hour_as_u32_or_default(now).expect("Cannot convert hour");
        let minute = self.minute_as_u32_or_default(now).expect("Cannot convert minute");
        let second = self.second_as_u32_or_default(now).expect("Cannot convert second");

        now.hour() == hour && now.minute() == minute && now.second() == second
    }

    fn hour_as_u32_or_default(&self, now: DateTime<Local>) -> Option<u32> {
        if self.hour == "*" {
            Some(now.hour())
        } else {
            self.hour.parse::<u32>().ok()
        }
    }

    fn minute_as_u32_or_default(&self, now: DateTime<Local>) -> Option<u32> {
        if self.minute == "*" {
            Some(now.minute())
        } else {
            self.minute.parse::<u32>().ok()
        }
    }

    fn second_as_u32_or_default(&self, now: DateTime<Local>) -> Option<u32> {
        if self.second == "*" {
            Some(now.second())
        } else {
            self.second.parse::<u32>().ok()
        }
    }
}

pub enum TaskStatus {
    NotReady,
    Success,
    Error(String),
}
