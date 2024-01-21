use serde::Deserialize;
use std::env::var;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub auto_commit: bool,
}

pub fn get_config() -> Config {
    // Get the value from the HOME variable
    let home = var("HOME").unwrap();
    // Read the config file (if exists)
    let config_file =
        fs::read_to_string(format!("{}/.config/baup/config.toml", home)).unwrap_or("".to_string());
    // Parse the config file
    let config: Config = match toml::from_str(&config_file) {
        Ok(file) => file,
        Err(_) => Config {
            path: "~/.baup/files.txt".to_string(),
            auto_commit: false,
        },
    };

    return config;
}
