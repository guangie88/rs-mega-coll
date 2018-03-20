use postgres::tls::native_tls::NativeTls;
use serde::de::{Deserialize, Deserializer};
use std::fs::File;

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
struct TlsHandshakeRep {
    domain: String,
    pub_key_path: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
enum TlsModeRep {
    None,
    Prefer(TlsHandshakeRep),
    Require(TlsHandshakeRep),
}

fn hs_to_tls(hs: &TlsHandshakeRep) -> NativeTls {
    let mut tls = NativeTls::new().unwrap();

    {
        let f = File::open(&hs.pub_key_path).unwrap();
        let connector = tls.connector_mut();
        connector.connect(&hs.domain, &f).unwrap();
    }

    tls
}

impl<'de> Deserialize<'de> for TlsModeNative {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: TlsModeRep = Deserialize::deserialize(d)?;

        Ok(match rep {
            TlsModeRep::None => TlsModeNative::None,
            TlsModeRep::Prefer(hs) => {
                let tls = hs_to_tls(&hs);
                TlsModeNative::Prefer(tls)
            }
            TlsModeRep::Require(hs) => {
                let tls = hs_to_tls(&hs);
                TlsModeNative::Require(tls)
            }
        })
    }
}
