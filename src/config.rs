use std::{fs::File, io::Read, path::PathBuf};

use eyre::{ensure, eyre};
use serde::Deserialize;

pub fn load_configuration() -> eyre::Result<Config> {
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
    File::open(config_dir.join("config.toml"))?.read_to_string(&mut config_contents)?;

    let config: Config = toml::from_str(&config_contents)?;

    Ok(config)
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
