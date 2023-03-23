use std::fmt::{Display, Formatter};
use crate::headers::Types;

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<String> for Types {
    fn from(value: String) -> Self {
        match value.as_str() {
            "_name" => Types::Name,
            "_key" => Types::Key,
            "_fields" => Types::Fields,
            "_in_memory" => Types::InMemory,
            "_declare" => Types::Declare,
            _ => Types::Name
        }
    }
}

impl Types {
    pub fn as_str(&self) -> &str {
        match self {
            Types::Name => "_name",
            Types::Key => "_key",
            Types::Fields => "_fields",
            Types::InMemory => "_in_memory",
            Types::Declare => "_declare"
        }
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}