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

pub enum TaskStatus {
    NotReady,
    Success,
    Error(String)
}
