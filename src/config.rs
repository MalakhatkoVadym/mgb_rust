use serde::Deserialize;
use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct MGBConfig {
    #[serde(
        default = "default_log_level",
        deserialize_with = "deserialize_log_level"
    )]
    pub log_level: Level,
    #[serde(default = "default_log_file_path")]
    pub log_file_path: String,
    #[serde(default = "default_database_url")]
    pub database_url: String,
    #[serde(default = "default_server_host")]
    pub server_host: String,
    #[serde(default = "default_server_port")]
    pub server_port: u16,
}

impl MGBConfig {
    pub fn parse_from_file(file_path: &str) -> Self {
        match std::fs::read_to_string(file_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => {
                    return config;
                }
                Err(e) => {
                    eprintln!(
                        "Failed to parse configuration file {}: {}, using default configuration",
                        file_path, e
                    );
                }
            },
            Err(e) => {
                eprintln!(
                    "Failed to read configuration file {}: {}, using default configuration",
                    file_path, e
                );
            }
        };
        MGBConfig::default()
    }
}

impl Default for MGBConfig {
    fn default() -> Self {
        MGBConfig {
            log_level: default_log_level(),
            log_file_path: default_log_file_path(),
            database_url: default_database_url(),
            server_host: default_server_host(),
            server_port: default_server_port(),
        }
    }
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let level_str: String = Deserialize::deserialize(deserializer)?;
    level_str.parse().map_err(serde::de::Error::custom)
}

fn default_log_level() -> Level {
    Level::INFO
}

pub fn default_log_file_path() -> String {
    "logs/mgb.log".to_string()
}

fn default_database_url() -> String {
    "sqlite:mgb.db".to_string()
}

fn default_server_host() -> String {
    "127.0.0.1".to_string()
}

fn default_server_port() -> u16 {
    3000
}
