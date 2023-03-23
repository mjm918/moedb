use fancy_regex::Regex;
use valico::json_dsl::{array, boolean, Builder, Param, string};
use crate::error::JqlError;
use crate::headers::{ActionType, DataTypes, Jql, JqlCommand, Types};

/// !```
/// {
///     "_action": "create-db | create-collection | get | delete | drop | drop-db | upsert | truncate | db-*",
///     "_database": "<your_database_name>",
///     "_collection": "<your_collection_name>",
///     "_body": "your json based on `_action`"
/// }
/// Create Store example
/// {
///     "_name": "<your_collection_name>",
///     "_key": "<your_field_name_that's_going_to_be_primary_key>",
///     "_in_memory":true|false,
///     "_fields":[
///         {
///             "_name":"<field_name>",
///             "_declare":"<declaration>"
///         }
///     ]
/// }
/// !```

impl Jql {

    pub fn type_declaration(q: &mut Param){
        q.allow_values(&[
            DataTypes::Int.to_string(),
            DataTypes::Uint.to_string(),
            DataTypes::Float.to_string(),
            DataTypes::String.to_string(),
            DataTypes::Boolean.to_string(),
            DataTypes::Date.to_string(),
            DataTypes::Time.to_string(),
            DataTypes::DateTime.to_string(),
            DataTypes::ArrayOfString.to_string(),
            DataTypes::ArrayOfBoolean.to_string(),
            DataTypes::ArrayOfInt.to_string(),
            DataTypes::ArrayOfUint.to_string(),
            DataTypes::ArrayOfFloat.to_string()
        ]);
    }

    pub fn naming_regx(p: &mut Param){
        p.coerce(string());
        p.regex(Regex::new(r"^[a-z|A-Z\-\d]{2,20}$").unwrap());
    }

    pub fn non_empty_regx(p: &mut Param){
        p.coerce(string());
        p.regex(Regex::new(r"^(?!\s*$).+").unwrap());
    }

    pub fn declare_time(p: &mut Param){
        p.regex(Regex::new(r"^(([0-1]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9])$").unwrap());
    }

    pub fn declare_date(p: &mut Param){
        p.regex(Regex::new(r"^\d197[0-9]|19[89][0-9]|20[0-9]{2}-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])$").unwrap());
    }

    pub fn declare_datetime(p: &mut Param){
        p.regex(Regex::new(r"^\d197[0-9]|19[89][0-9]|20[0-9]{2}-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01]) (([0-1]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9])$").unwrap());
    }

    pub fn jql_command_parser(command: &str) -> Result<(), JqlError>{
        let parser = serde_json::from_str::<JqlCommand>(command);
        if parser.is_err() {
            return Err(JqlError::DocumentParseError);
        }
        let parsed: JqlCommand = parser.unwrap();
        let action = ActionType::from(parsed._action);
        match action {
            ActionType::Create => {
                //
            }
            ActionType::CreateDb => {

            }
            ActionType::Get => {

            }
            ActionType::Upsert => {

            }
            ActionType::Delete => {

            }
            ActionType::Drop => {

            }
            ActionType::DropDb => {

            }
            ActionType::DbList => {

            }
            ActionType::Truncate => {

            }
        }
        Ok(())
    }
}