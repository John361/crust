mod task;

use crate::task::{Schedule, Task};

fn main() {
    let schedule1 = Schedule { minute: 9, hour: 22 };
    let schedule2 = Schedule { minute: 10, hour: 22 };

    let task1 = Task { schedule: schedule1, command: "hello".to_string() };
    let task2 = Task { schedule: schedule2, command: "world".to_string() };
    let tasks = vec![task1, task2];

    loop {
        for task in &tasks {
            task.execute();
        }
    }
}
