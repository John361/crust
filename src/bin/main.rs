use crust_lib::config::CrustConfig;
use crust_lib::task::TaskStatus;

fn main() -> anyhow::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default())?;
    let config = CrustConfig::load()?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        for task in &config.tasks {
            let result = task.execute()?; // TODO: open new thread for execution
            match result {
                TaskStatus::Success => {
                    log::info!("Task successfully executed.");
                }

                TaskStatus::Error(error) => {
                    log::error!("Task execution error: {}", error);
                }

                TaskStatus::NotReady => {}
            }
        }
    }
}
