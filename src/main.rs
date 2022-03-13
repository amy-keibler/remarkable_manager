use crate::config::{ConfigurationFromFile, ConfigurationSource};
use eyre::Context;

mod config;
mod templates;

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    let config = ConfigurationFromFile::load_configuration()
        .wrap_err("Could not load configuration from the application's configuration folder")?;

    Ok(())
}
