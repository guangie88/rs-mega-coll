use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Auth<'a> {
    Password(Cow<'a, str>),
    Keytab(Cow<'a, str>),
}

#[derive(Deserialize, Debug)]
pub struct KInitConfig<'a> {
    pub login: String,
    pub auth: Auth<'a>,
}
