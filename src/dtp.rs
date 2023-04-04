use std::fmt::{Display, Formatter, Result};
use crate::hdrs::DataTypes;

impl Display for DataTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<String> for DataTypes {
    fn from(value: String) -> Self {
        match value.as_str() {
            "int" => DataTypes::Int,
            "uint" => DataTypes::Uint,
            "float" => DataTypes::Float,
            "string" => DataTypes::String,
            "boolean" => DataTypes::Boolean,
            "date" => DataTypes::Date,
            "time" => DataTypes::Time,
            "datetime" => DataTypes::DateTime,
            "int[]" => DataTypes::ArrayOfInt,
            "uint[]" => DataTypes::ArrayOfUint,
            "float[]" => DataTypes::ArrayOfFloat,
            "string[]" => DataTypes::ArrayOfString,
            &_ => DataTypes::String
        }
    }
}

impl DataTypes {
    pub fn to_string(&self) -> String {
        match self {
            DataTypes::Int => "int".to_string(),
            DataTypes::Uint => "uint".to_string(),
            DataTypes::Float => "float".to_string(),
            DataTypes::String => "string".to_string(),
            DataTypes::Boolean => "boolean".to_string(),
            DataTypes::Date => "date".to_string(),
            DataTypes::Time => "time".to_string(),
            DataTypes::DateTime => "datetime".to_string(),
            DataTypes::ArrayOfString => "string[]".to_string(),
            DataTypes::ArrayOfInt => "int[]".to_string(),
            DataTypes::ArrayOfUint => "uint[]".to_string(),
            DataTypes::ArrayOfFloat => "float[]".to_string(),
            DataTypes::ArrayOfBoolean => "boolean[]".to_string()
        }
    }
}

impl PartialEq for DataTypes {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}
