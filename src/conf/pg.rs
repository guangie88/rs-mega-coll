use postgres::tls::native_tls::NativeTls;
use serde::de::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub connection_url: String,
    pub estimated_cap: u64,
    pub tls_mode: TlsModeNative,
}

#[derive(Debug)]
pub enum TlsModeNative {
    None,
    Prefer(NativeTls),
    Require(NativeTls),
}

#[derive(Deserialize, Debug)]
enum TlsModeRep {
    None,
    Prefer,
    Require,
}

impl<'de> Deserialize<'de> for TlsModeNative {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: TlsModeRep = Deserialize::deserialize(d)?;

        Ok(match rep {
            TlsModeRep::None => TlsModeNative::None,
            TlsModeRep::Prefer => {
                TlsModeNative::Prefer(NativeTls::new().unwrap())
            }
            TlsModeRep::Require => {
                TlsModeNative::Require(NativeTls::new().unwrap())
            }
        })
    }
}
