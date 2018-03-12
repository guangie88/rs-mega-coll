use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub connection_url: String,
    pub entry_expiry_duration: Duration,
}
