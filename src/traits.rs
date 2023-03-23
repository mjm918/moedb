use serde_json::Value;
use crate::error::JqlError;

pub trait JqlValueParser {
    fn new(base: String) -> Self;
    fn parse(&self, values: String) -> Result<(), JqlError>;
    fn parse_with_value(&self, values: serde_json::error::Result<Value>) -> Result<(), JqlError>;
}

pub trait JqlSchemaParser {
    fn new() -> Self;
    fn parse(&self, values: String) -> Result<(), JqlError>;
    fn parse_with_value(&self, values: serde_json::error::Result<Value>) -> Result<(), JqlError>;
}

