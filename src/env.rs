use anyhow::{Result};
use std::fs;
use serde_derive::{Deserialize};
use tracing::error;
use crate::error::EnvReadError;
use crate::var::ENV_PATH;

#[derive(Debug, Deserialize, Clone)]
pub struct BaseConfig {
    pub moedb: MoeDb
}

#[derive(Debug, Deserialize, Clone)]
pub struct MoeDb {
    pub db_path: String,
    pub log_path: String,
    pub default_auth: Vec<String>
}

pub fn env() -> Result<MoeDb,EnvReadError> {
    let content = fs::read_to_string(ENV_PATH);
    if content.is_err() {
        let err = content.err().unwrap().to_string();
        error!("{}",err);
        return Err(EnvReadError::NoContent(err));
    }
    let sys_cfg: Result<BaseConfig,toml::de::Error> = toml::from_str(content.unwrap().as_str());
    if sys_cfg.is_err() {
        let err = sys_cfg.err().unwrap().to_string();
        error!("{}",err);
        return Err(EnvReadError::InvalidToml(err));
    }
    let moedb = sys_cfg.unwrap().moedb;
    let mtd_db = fs::metadata(moedb.db_path.as_str());
    if mtd_db.is_err() || !mtd_db.as_ref().unwrap().is_dir() {
        let err = mtd_db.err().unwrap().to_string();
        error!("{}",err);
        return Err(EnvReadError::DbPathNotValid(err));
    }
    let mtd_log = fs::metadata(moedb.log_path.as_str());
    if mtd_log.is_err() || !mtd_log.as_ref().unwrap().is_dir() {
        let err = mtd_log.err().unwrap().to_string();
        error!("{}",err);
        return Err(EnvReadError::LogFileNotValid(err));
    }
    Ok(moedb)
}