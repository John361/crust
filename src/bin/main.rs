use crust_lib::config::CrustConfig;
use crust_lib::task::TaskStatus;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default())?;
    let config = CrustConfig::load()?;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        for task in &config.tasks {
            let task = task.clone();
            let notifiers = config.notifiers.clone();

            tokio::task::spawn(async move {
                let result = task.execute().expect("Task execution failed");
                match result {
                    TaskStatus::Success => {
                        log::info!("Task successfully executed.");
                    }

                    TaskStatus::Error(error) => {
                        let message = format!("Task execution error: {}", error);
                        log::error!("{}", message);

                        if let Some(notifiers) = notifiers {
                            notifiers.notify("Crusty".to_string(), message).await
                                .expect("Failed to notify");
                        }
                    }

                    TaskStatus::NotReady => {}
                }
            });
        }
    }
}
