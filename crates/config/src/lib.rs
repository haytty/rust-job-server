use getset::Getters;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use thiserror::Error;

pub trait ConfigPath {
    fn config_path(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct Config {
    #[get = "pub"]
    queue: ConfigQueue,
    #[get = "pub"]
    server: ConfigServer,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
#[serde(rename = "queue")]
pub struct ConfigQueue {
    #[get = "pub"]
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
#[serde(rename = "server")]
pub struct ConfigServer {
    #[get = "pub"]
    read_timeout: u32,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("File read error: {0}")]
    FileReadError(#[from] io::Error),

    #[error("TOML parse error: {0}")]
    TomlParseError(#[from] toml::de::Error),
}

pub fn load_config<T>(path: T) -> Result<Config, ConfigError>
where
    T: ConfigPath,
{
    let config_content: String =
        fs::read_to_string(path.config_path()).map_err(ConfigError::FileReadError)?;
    let config: Config = toml::from_str(&config_content).map_err(ConfigError::TomlParseError)?;

    Ok(config)
}
