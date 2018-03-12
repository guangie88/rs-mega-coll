#[derive(Deserialize, Debug)]
pub struct DfConfig {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct CopyConfig {
    path: String,
    matches: Vec<String>,
    copy_to: String,
}
