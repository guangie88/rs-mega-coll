use serde::de::DeserializeOwned;
use serde_humantime;
use std::time::Duration;
use structopt::StructOpt;

pub trait ArgConf: StructOpt {
    fn conf(&self) -> &str;
}

pub trait Conf: DeserializeOwned {
    fn general(&self) -> &Config;
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_conf_path: Option<String>,
    pub lock_file: String,
    #[serde(with = "serde_humantime", default)]
    pub repeat_delay: Option<Duration>,
}
