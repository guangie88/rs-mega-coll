use conf::app::{ArgConf, Conf};
use conf::fluentd;
use error::{Error, ErrorKind};
use error::custom::PathError;
use failure::{Fail, ResultExt};
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use fruently::retry_conf::RetryConf;
use log4rs;
use serde::ser::Serialize;
use simple_logger;
use std::fmt::{Debug, Display};
use std::path::Path;
use toml;
use util::fs::read_from_file;

pub fn create_and_check_fluent<T, K>(
    conf: &fluentd::Config,
    init_msg: T,
) -> Result<Fluent<&String>, Error<K>>
where
    T: Debug + Serialize,
    K: From<ErrorKind> + Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    let fluent_conf = RetryConf::new()
        .max(conf.try_count)
        .multiplier(conf.multiplier);

    let fluent_conf = match conf.store_file_path {
        Some(ref store_file_path) => {
            fluent_conf.store_file(Path::new(store_file_path).to_owned())
        }
        None => fluent_conf,
    };

    let fluent =
        Fluent::new_with_conf(&conf.address, conf.tag.as_str(), fluent_conf);

    fluent
        .clone()
        .post(&init_msg)
        .context(ErrorKind::FluentInitCheck)?;

    Ok(fluent)
}

pub fn init_config<A, C, K>() -> Result<C, Error<K>>
where
    A: ArgConf,
    C: Conf,
    K: From<ErrorKind> + Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    let arg_conf = A::from_args();
    let conf: C = read_config_file(arg_conf.conf())?;

    match conf.general().log_conf_path {
        Some(ref log_conf_path) => {
            log4rs::init_file(log_conf_path, Default::default())
                .map_err(|e| PathError::new(log_conf_path, e))
                .context(ErrorKind::SpecializedLoggerInit)?
        }
        None => simple_logger::init().context(ErrorKind::DefaultLoggerInit)?,
    }

    Ok(conf)
}

pub fn print_run_status<M, T, K>(res: &Result<T, Error<K>>, success_msg: M)
where
    M: Display,
    K: From<ErrorKind> + Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    match *res {
        Ok(_) => info!("{}", success_msg),
        Err(ref e) => {
            error!("{}", e);
        }
    }
}

pub fn read_config_file<P, C, K>(conf_path: P) -> Result<C, Error<K>>
where
    P: AsRef<Path>,
    C: Conf,
    K: From<ErrorKind> + Copy + Clone + Eq + PartialEq + Debug + Fail,
{
    let conf_path = conf_path.as_ref();

    let config: C = toml::from_str(&read_from_file(conf_path)?)
        .map_err(|e| PathError::new(conf_path, e))
        .context(ErrorKind::TomlConfigParse)?;

    Ok(config)
}
