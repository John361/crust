use crust_lib::config::CrustConfig;

fn main() -> anyhow::Result<()> {
    let config = CrustConfig::load()?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        for task in &config.tasks {
            task.execute(); // TODO: open new thread for execution
        }
    }
}
