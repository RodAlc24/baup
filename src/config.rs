use serde::Deserialize;
use std::env::var;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: Option<String>,
    pub auto_commit: Option<bool>,
    pub hooks: Option<Hooks>,
}

#[derive(Deserialize, Debug)]
pub struct Hooks {
    import_hook: Option<String>,
    export_hook: Option<String>,
}

impl Config {
    fn default_config() -> Config {
        Config {
            path: Some("~/.baup/files.txt".to_string()),
            auto_commit: Some(false),
            hooks: Some(Hooks {
                import_hook: None,
                export_hook: None,
            }),
        }
    }
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

    let loaded_hooks = loaded_config.hooks.unwrap_or(Hooks {
        import_hook: None,
        export_hook: None,
    });

    let default_config = Config::default_config();
    let config = Config {
        path: match loaded_config.path {
            Some(path) => Some(path),
            None => default_config.path,
        },
        auto_commit: match loaded_config.auto_commit {
            Some(auto_commit) => Some(auto_commit),
            None => default_config.auto_commit,
        },
        hooks: Some(Hooks {
            import_hook: loaded_hooks.import_hook.clone(),
            export_hook: loaded_hooks.export_hook.clone(),
        }),
    };

    config
}
