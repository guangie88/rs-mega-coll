use serde_humantime;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub connection_url: String,
    #[serde(with = "serde_humantime", default)]
    pub entry_expiry_duration: Duration,
}
