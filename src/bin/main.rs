use crust_lib::task::{Schedule, Task, Tasks};

fn main() {
    let schedule1 = Schedule { hour: 22, minute: 44, second: 30 };
    let schedule2 = Schedule { hour: 22, minute: 44, second: 35 };

    let task1 = Task { schedule: schedule1, command: "hello".to_string() };
    let task2 = Task { schedule: schedule2, command: "world".to_string() };
    let tasks = Tasks { tasks: vec![task1, task2] };

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        for task in &tasks.tasks {
            task.execute(); // TODO: open new thread for execution
        }
    }
}
