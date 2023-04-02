use std::fmt::{Display, Formatter};
use crate::headers::ActionType;

impl Display for ActionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<&str> for ActionType {
    fn from(value: &str) -> Self {
        match value {
            "create-db" => ActionType::CreateDb,
            "create-collection" => ActionType::Create,
            "get" => ActionType::Get,
            "upsert" => ActionType::Upsert,
            "delete" => ActionType::Delete,
            "drop-collection" => ActionType::Drop,
            "drop-db" => ActionType::DropDb,
            "truncate" => ActionType::Truncate,
            "db-*" => ActionType::DbList,
            "col-*" => ActionType::ColList,
            _ => ActionType::Unknown
        }
    }
}

impl ActionType {
    pub fn to_string(&self) -> String {
        match self {
            ActionType::Create => "create-collection".to_string(),
            ActionType::CreateDb => "create-db".to_string(),
            ActionType::Get => "get".to_string(),
            ActionType::Upsert => "upsert".to_string(),
            ActionType::Delete => "delete".to_string(),
            ActionType::Drop => "drop-collection".to_string(),
            ActionType::DropDb => "drop-db".to_string(),
            ActionType::DbList => "db-*".to_string(),
            ActionType::Truncate => "truncate".to_string(),
            ActionType::ColList => "col-*".to_string(),
            _ => "".to_string()
        }
    }
}

impl PartialEq for ActionType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}