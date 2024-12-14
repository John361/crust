use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'l', long = "log-path", value_name = "LOG_PATH")]
    pub log_path: String,

    #[arg(short = 'c', long = "config-path", value_name = "CONFIG_PATH")]
    pub config_path: String,
}

impl Cli {
    pub fn load() -> Self {
        Self::parse()
    }
}
