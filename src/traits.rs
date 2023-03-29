use serde_json::Value;
use crate::error::JqlError;

pub trait JqlValueParser {
    fn new_value_parser(base: String) -> Self;
    fn parse_value(&self, values: String) -> Result<(), JqlError>;
    fn parse_value_with_json(&self, values: serde_json::error::Result<Value>) -> Result<(), JqlError>;
}

pub trait JqlSchemaParser {
    fn new_schema_parser() -> Self;
    fn parse_schema(&self, values: String) -> Result<(), JqlError>;
    fn parse_schema_with_json(&self, values: serde_json::error::Result<Value>) -> Result<(), JqlError>;
}

