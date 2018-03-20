use postgres::tls::native_tls::NativeTls;
use serde::de::{Deserialize, Deserializer};
// use std::net::TcpStream;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub connection_url: String,
    pub estimated_cap: u64,
    pub tls_mode: TlsModeRep,
}

// #[derive(Debug)]
// pub enum TlsModeNative {
//     None,
//     Prefer(NativeTls),
//     Require(NativeTls),
// }

#[derive(Deserialize, Debug)]
pub struct TlsHandshakeRep {
    pub domain: String,
    pub url: String,
    pub pem_files: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum TlsModeRep {
    None,
    Prefer(TlsHandshakeRep),
    Require(TlsHandshakeRep),
}

// fn hs_to_tls(hs: &TlsHandshakeRep) -> NativeTls {
//     let mut tls = NativeTls::new().unwrap();

//     {
//         let stream = TcpStream::connect(&hs.url).unwrap();
//         let connector = tls.connector_mut();
//         connector.connect(&hs.domain, &stream).unwrap();
//     }

//     tls
// }

// impl<'de> Deserialize<'de> for TlsModeNative {
//     fn deserialize<D>(d: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let rep: TlsModeRep = Deserialize::deserialize(d)?;

//         Ok(match rep {
//             TlsModeRep::None => TlsModeNative::None,
//             TlsModeRep::Prefer(hs) => {
//                 let tls = hs_to_tls(&hs);
//                 TlsModeNative::Prefer(tls)
//             }
//             TlsModeRep::Require(hs) => {
//                 let tls = hs_to_tls(&hs);
//                 TlsModeNative::Require(tls)
//             }
//         })
//     }
// }
