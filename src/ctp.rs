use std::fmt::{Display, Formatter};
use crate::headers::CommandType;

impl Display for CommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<String> for CommandType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "_action" => CommandType::Action,
            "_body" => CommandType::Body,
            "_database" => CommandType::Database,
            "_collection" => CommandType::Store,
            _ => CommandType::Action
        }
    }
}

impl CommandType {
    pub fn as_str(&self) -> &str {
        match self {
            CommandType::Action => "_action",
            CommandType::Body => "_body",
            CommandType::Database => "_database",
            CommandType::Store => "_collection"
        }
    }
}

impl PartialEq for CommandType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}
