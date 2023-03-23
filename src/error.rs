use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum JqlError {
    #[error("document property data type check error `{0}`")]
    DocumentPropertyDataTypeError(String),
    #[error("document is not a json")]
    DocumentParseError,
    #[error("no fields provided in the schema `{0}`")]
    NoFieldsProvided(String),
    #[error("malformed schema")]
    MalformedSchema,
    #[error("schema property value error `{0}`")]
    SchemaPropertyTypeError(String)
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum EnvReadError {
    #[error("failed to read config file `{0}`")]
    NoContent(String),
    #[error("invalid toml format `{0}`")]
    InvalidToml(String),
    #[error("log file path is not valid `{0}`")]
    LogFileNotValid(String),
    #[error("database path is not valid `{0}`")]
    DbPathNotValid(String)
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum MoeDbError {
    #[error("cf error `{0}`")]
    CfError(String),
    #[error("cf creating error `{0}`")]
    CfCreateError(String),
    #[error("big endian error `{0}`")]
    BigEndianError(String),
    #[error("error while getting key from db")]
    GetError
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum OpsError {
    #[error("create db error `{0}`")]
    CreateDbError(String),
    #[error("create collection error `{0}`")]
    CreateCollectionError(String)
}