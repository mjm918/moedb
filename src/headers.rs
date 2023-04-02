use std::sync::Arc;
use rayon::ThreadPool;
use rocksdb::{DBWithThreadMode, MultiThreaded};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use valico::json_dsl::Builder;
use crate::env;
use crate::error::TrxError;

pub type TKey = Vec<u8>;
pub type TValue = Vec<u8>;

pub type MoeDbMode = DBWithThreadMode<MultiThreaded>;

pub struct MoeDb {
    pub exec: Exec
}

pub struct Exec {
    pub pool: ThreadPool,
    pub env: Arc<env::MoeDb>,
    pub trx: Arc<Trx>
}

pub struct Trx {
    pub db: Arc<MoeDbMode>,
    pub env: Arc<env::MoeDb>
}

#[derive(Clone)]
pub struct ParsedStatement {
    pub cmd: Option<ActionType>,
    pub db: String,
    pub store: String,
    pub pbs_data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub time_taken: String,
    pub error: bool,
    pub message: String,
    pub data: Option<Vec<Value>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbRes {
    pub data: Option<Vec<Value>>,
    pub error: Option<TrxError>,
}

pub struct Jql {
    pub prs: Builder,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JqlSchema {
    pub _name: String,
    pub _key: String,
    pub _fields: Vec<JqlSchemaFields>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JqlSchemaFields {
    pub _name: String,
    pub _declare: String,
    pub _optional: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct JqlCommand {
    pub _action: String,
    pub _body: Option<Value>,
    pub _database: Option<String>,
    pub _collection: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum DataTypes {
    Int,
    Uint,
    Float,
    String,
    Boolean,
    Date,
    DateTime,
    Time,
    ArrayOfString,
    ArrayOfInt,
    ArrayOfUint,
    ArrayOfFloat,
    ArrayOfBoolean,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Types {
    Name,
    Key,
    Fields,
    InMemory,
    Declare,
}

#[derive(Clone, Ord, PartialOrd, Eq, Debug)]
pub enum ActionType {
    Create,
    CreateDb,
    Get,
    Upsert,
    Delete,
    Drop,
    DropDb,
    DbList,
    ColList,
    Truncate,
    Unknown
}

#[derive(Clone, Ord, PartialOrd, Eq, Debug)]
pub enum CommandType {
    Action,
    Body,
    Database,
    Store,
}
