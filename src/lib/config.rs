use anyhow::Context;
use serde::Deserialize;

use crate::task::Task;

#[derive(Debug, Deserialize)]
pub struct CrustConfig {
    pub tasks: Vec<Task>,
}

impl CrustConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path = "config/crust.yaml";

        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .context(format!("Failed to build config from {0}", path))?;

        let crust_config = config
            .try_deserialize::<CrustConfig>()
            .context("Failed to deserialize into AppConfig")?;

        Ok(crust_config)
    }
}
