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
    SchemaPropertyTypeError(String),
    #[error("no database name provided")]
    NoDatabaseProvided,
    #[error("no collection or malformed name provided")]
    NoCollectionProvided,
    #[error("no schema or malformed provided")]
    NoSchemaProvided,
    #[error("unknown query")]
    UnknownQuery,
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum EnvReadError {
    #[error("failed to read config file `{0}`")]
    NoContent(String),
    #[error("invalid toml format `{0}`")]
    InvalidToml(String),
    #[error("log path is not valid `{0}`")]
    LogPathNotValid(String),
    #[error("database path is not valid `{0}`")]
    DbPathNotValid(String),
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
    GetError,
    #[error("invalid query `{0}`")]
    QueryError(String),
    #[error("transaction error `{0}`")]
    TransactionError(String),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum TrxError {
    #[error("error creating db `{0}`")]
    CreateDbError(String),
    #[error("error creating collection `{0}`")]
    CreateCollectionError(String),
    #[error("error truncating collection `{0}`")]
    TruncateCollectionError(String),
    #[error("error dropping collection `{0}`")]
    DropCollectionError(String),
    #[error("unknown error")]
    UnknownError
}

pub enum NativeError {
    CfAlreadyExists,
    CfDoesNotExists
}

pub fn from_native_error(err: String) -> Option<NativeError> {
    if err.contains("Invalid column family") {
        return Some(NativeError::CfDoesNotExists);
    }
    None
}