use serde::Deserialize;
use std::env::var;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(default = "default_auto_commit")]
    pub auto_commit: bool,
    #[serde(default = "default_hooks")]
    pub hooks: Hooks,
}

#[derive(Deserialize, Debug)]
pub struct Hooks {
    pub import_hook: Option<String>,
    pub export_hook: Option<String>,
}

impl Config {
    fn default_config() -> Config {
        Config {
            path: "~/.baup/files.txt".to_string(),
            auto_commit: false,
            hooks: Hooks {
                import_hook: None,
                export_hook: None,
            },
        }
    }
}

fn default_path() -> String {
    return "~/.baup/files.txt".to_string();
}

fn default_auto_commit() -> bool {
    return false;
}

fn default_hooks() -> Hooks {
    return Hooks {
        import_hook: None,
        export_hook: None,
    };
}

pub fn get_config() -> Config {
    // Get the value from the HOME variable
    let home = var("HOME").unwrap();
    // Read the config file (if exists)
    let config_file_path = format!("{}/.config/baup/config.toml", home);
    let config_file = fs::read_to_string(config_file_path).unwrap_or("".to_string());
    // Parse the config file
    let loaded_config: Config = match toml::from_str(&config_file) {
        Ok(file) => file,
        Err(_) => Config::default_config(),
    };

    loaded_config
}
