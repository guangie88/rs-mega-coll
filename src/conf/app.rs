use std::time::Duration;
use serde_humantime;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_conf_path: Option<String>,
    pub lock_file: String,
    #[serde(with = "serde_humantime", default)]
    pub repeat_delay: Option<Duration>,
}
