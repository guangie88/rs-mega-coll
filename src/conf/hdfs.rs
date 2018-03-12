#[derive(Deserialize, Debug)]
pub struct DfConfig {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct CopyConfig {
    pub path: String,
    pub matches: Vec<String>,
    pub copy_to: String,
}
