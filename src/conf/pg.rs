use error::{ErrorKind, Result};
use failure::ResultExt;
use filebuffer::FileBuffer;
use native_tls::{Certificate, TlsConnector};
use postgres::tls::native_tls::NativeTls;
use serde::de::{self, Deserialize, Deserializer};
use std;

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
pub struct TlsHandshakeRep {
    pub pem_files: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum TlsModeRep {
    None,
    Prefer(TlsHandshakeRep),
    Require(TlsHandshakeRep),
}

fn to_native_tls(hs: &TlsHandshakeRep) -> Result<NativeTls> {
    let mut conn_builder =
        TlsConnector::builder().context(ErrorKind::TlsConnectorBuilder)?;

    for pem_file in &hs.pem_files {
        let pem_buf = FileBuffer::open(pem_file)
            .context(ErrorKind::PemCertificateFileOpen)?;

        conn_builder
            .add_root_certificate(
                Certificate::from_pem(&pem_buf)
                    .context(ErrorKind::PemCertificateRead)?,
            )
            .context(ErrorKind::TlsConnectorBuilderAddRootCertificate)?;
    }

    let conn = conn_builder
        .build()
        .context(ErrorKind::TlsConnectorBuilderBuild)?;

    Ok(NativeTls::from(conn))
}

impl<'de> Deserialize<'de> for TlsModeNative {
    fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: TlsModeRep = Deserialize::deserialize(d)?;

        let to_tls = |hs| -> std::result::Result<NativeTls, D::Error> {
            to_native_tls(&hs).map_err(de::Error::custom)
        };

        Ok(match rep {
            TlsModeRep::None => TlsModeNative::None,
            TlsModeRep::Prefer(hs) => TlsModeNative::Prefer(to_tls(hs)?),
            TlsModeRep::Require(hs) => TlsModeNative::Require(to_tls(hs)?),
        })
    }
}
