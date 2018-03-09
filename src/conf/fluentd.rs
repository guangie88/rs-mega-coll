#[derive(Deserialize, Debug)]
pub struct Config {
    pub address: String,
    pub tag: String,
    pub try_count: u64,
    pub multiplier: f64,
    pub store_file_path: Option<String>,
}
