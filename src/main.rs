use eyre::bail;

mod config;
mod templates;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let config = config::load_configuration()?;
    println!("Got configuration: {config:?}");

    if config.backup_folder.exists() && config.backup_folder.is_dir() {
        println!("Backup folder set up correctly");
    } else {
        bail!(
            "Backup folder does not exist: {}",
            config.backup_folder.to_string_lossy()
        );
    }

    println!("Hello, Amy!");

    Ok(())
}
