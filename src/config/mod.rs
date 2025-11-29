use self::colors::ColorConfig;
use serde::Deserialize;

use std::env::home_dir;
use std::fs::read_to_string;
use std::sync::OnceLock;

pub mod colors;

pub const REFRESH_RATE_MILLIS: u64 = 1000;
pub const GRAPH_X_AXIS_WINDOW_IN_SECONDS: usize = 120;
pub const MESSAGE_EXPIRATION_IN_SECONDS: u64 = 10;

const DEFAULT_CONFIG_FILE: &str = ".config/mltop/config.toml";

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    pub colors: ColorConfig,
}

impl Config {
    pub fn get() -> Config {
        let Some(home) = home_dir() else {
            return Config::default();
        };

        let config_file = home.join(DEFAULT_CONFIG_FILE);
        match config_file.is_file() {
            false => Self::default(),
            true => {
                let Ok(contents) = read_to_string(&config_file) else {
                    return Config::default();
                };
                toml::from_str(&contents).unwrap_or(Config::default())
            }
        }
    }
}

pub fn init_config() {
    let config = Config::get();
    CONFIG
        .set(config)
        .expect("config should be initialized only once");
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("config should be initialized")
}
