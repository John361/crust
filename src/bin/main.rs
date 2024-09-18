use crust_lib::config::CrustConfig;
use crust_lib::task::TaskStatus;

fn main() -> anyhow::Result<()> {
    let config = CrustConfig::load()?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        for task in &config.tasks {
            let result = task.execute()?; // TODO: open new thread for execution
            match result {
                TaskStatus::Success => {
                    println!("Task successfully executed.");
                }

                TaskStatus::Error(error) => {
                    println!("Task execution error: {}", error);
                }

                TaskStatus::NotReady => {}
            }
        }
    }
}
