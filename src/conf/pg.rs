#[derive(Deserialize, Debug)]
pub struct Config {
    pub connection_url: String,
    pub estimated_cap: u64,
}
