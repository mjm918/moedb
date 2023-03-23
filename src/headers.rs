use std::sync::Arc;
use rocksdb::{DBWithThreadMode, MultiThreaded, Options};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use valico::json_dsl::Builder;

pub type MoeDbMode = DBWithThreadMode<MultiThreaded>;

pub struct MoeDb<'a> {
    pub db: Arc<MoeDbMode>,
    pub opts: Arc<Options>,
    pub path: &'a str,
    pub log: &'a str
}

pub struct Exec {
    pub db: Arc<MoeDbMode>,
    pub opts: Arc<Options>,
}

pub struct Ops {
    pub db: Arc<MoeDbMode>,
    pub opts: Arc<Options>,
}

#[derive(Clone)]
pub struct ParsedStatement {
    pub cmd: Option<ActionType>,
    pub db: String,
    pub store: String,
    pub pbs_data: String
}

#[derive(Clone)]
pub struct Response {
    pub time_taken: String,
    pub error: bool,
    pub message: String,
    pub data: Option<Value>
}

pub struct Jql {
    pub prs: Builder
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JqlSchema {
    pub _name: String,
    pub _key: String,
    pub _fields: Vec<JqlSchemaFields>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JqlSchemaFields {
    pub _name: String,
    pub _declare: String,
    pub _optional: Option<String>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JqlCommand {
    pub _action: String,
    pub _body: Option<JqlSchema>,
    pub _database: Option<String>,
    pub _collection: Option<String>
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
    ArrayOfBoolean
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Types {
    Name,
    Key,
    Fields,
    InMemory,
    Declare
}

#[derive(Clone,Ord, PartialOrd, Eq, Debug)]
pub enum ActionType {
    Create,
    CreateDb,
    Get,
    Upsert,
    Delete,
    Drop,
    DropDb,
    DbList,
    Truncate
}

#[derive(Clone,Ord, PartialOrd, Eq, Debug)]
pub enum CommandType {
    Action,
    Body,
    Database,
    Store
}
