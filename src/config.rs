use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use eyre::{ensure, eyre, Context};
use serde::Deserialize;

pub trait ConfigurationSource {
    fn load_configuration() -> eyre::Result<Config>;
}

pub struct ConfigurationFromFile;

impl ConfigurationSource for ConfigurationFromFile {
    fn load_configuration() -> eyre::Result<Config> {
        tracing::info!("Loading configuration");
        let project_dirs = directories::ProjectDirs::from("net", "amy-k", "remarkable_backup")
            .ok_or(eyre!("Could not construct project directory"))?;
        let config_dir = project_dirs.config_dir();

        ensure!(
            config_dir.exists() && config_dir.is_dir(),
            eyre!(
                "Configuration directory does not exist: {}",
                config_dir.to_string_lossy()
            )
        );

        let mut config_contents = String::new();
        let config_file = config_dir.join("config.toml");
        File::open(&config_file)
            .wrap_err(format!(
                "Attempted to open configuration file: {}",
                config_file.to_string_lossy()
            ))?
            .read_to_string(&mut config_contents)
            .wrap_err(format!(
                "Attempted to read configuration: {}",
                config_file.to_string_lossy()
            ))?;

        let config: Config = toml::from_str(&config_contents).wrap_err(format!(
            "Attempted to process configuration as TOML: {}",
            config_file.to_string_lossy()
        ))?;

        if config.backup_folder.exists() && config.backup_folder.is_dir() {
            tracing::debug!("Configuration's backup folder set up correctly");
        } else {
            fs::create_dir_all(&config.backup_folder).wrap_err(format!(
                "Could not create configuration's backup folder: {}",
                config.backup_folder.to_string_lossy()
            ))?;
        }
        tracing::info!("Loaded configuration");
        tracing::debug!("Configuration: {:?}", config);
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ssh_host: String,
    pub backup_folder: PathBuf,
    pub custom_templates: Option<Vec<CustomTemplate>>,
}

#[derive(Debug, Deserialize)]
pub struct CustomTemplate {
    pub name: String,
    pub filename: String,
    pub categories: Vec<String>,
}
