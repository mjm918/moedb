use fancy_regex::Regex;
use valico::json_dsl::{Param, string};
use crate::error::JqlError;
use crate::func::is_naming_ok;
use crate::headers::{ActionType, DataTypes, Jql, JqlCommand};
use crate::traits::{JqlSchemaParser};

/// !```
/// {
///     "_action": "create-db | create-collection | get | delete | drop-collection | drop-db | upsert | truncate | db-*",
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
    pub fn type_declaration(q: &mut Param) {
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

    pub fn naming_regx(p: &mut Param) {
        p.coerce(string());
        p.regex(Regex::new(r"^[a-z|A-Z][a-z|A-Z\-\d]{2,20}$").unwrap());
    }

    pub fn non_empty_regx(p: &mut Param) {
        p.coerce(string());
        p.regex(Regex::new(r"^(?!\s*$).+").unwrap());
    }

    pub fn declare_time(p: &mut Param) {
        p.regex(Regex::new(r"^(([0-1]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9])$").unwrap());
    }

    pub fn declare_date(p: &mut Param) {
        p.regex(Regex::new(r"^\d197[0-9]|19[89][0-9]|20[0-9]{2}-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])$").unwrap());
    }

    pub fn declare_datetime(p: &mut Param) {
        p.regex(Regex::new(r"^\d197[0-9]|19[89][0-9]|20[0-9]{2}-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01]) (([0-1]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9])$").unwrap());
    }

    pub fn parse(command: &str) -> Result<JqlCommand, JqlError> {
        let parser = serde_json::from_str::<JqlCommand>(command);
        if parser.is_err() {
            return Err(JqlError::DocumentParseError);
        }
        let parsed = parser.unwrap();
        let to_return = parsed.clone();
        return match ActionType::from(parsed._action.as_str()) {
            ActionType::Create => {
                match Jql::is_create_collection_ok(&to_return) {
                    Ok(_) => Ok(to_return),
                    Err(er) => Err(er)
                }
            }
            ActionType::CreateDb => {
                match Jql::is_db_ok(&to_return) {
                    Ok(_) => Ok(to_return),
                    Err(er) => Err(er)
                }
            }
            ActionType::Get => {
                todo!()
            }
            ActionType::Upsert => {
                todo!()
            }
            ActionType::Delete => {
                todo!()
            }
            ActionType::Drop => {
                match Jql::is_collection_ok(&to_return) {
                    Ok(_) => Ok(to_return),
                    Err(er) => Err(er)
                }
            }
            ActionType::DropDb => {
                match Jql::is_db_ok(&to_return) {
                    Ok(_) => Ok(to_return),
                    Err(er) => Err(er)
                }
            }
            ActionType::DbList => {
                todo!()
            }
            ActionType::Truncate => {
                match Jql::is_collection_ok(&to_return) {
                    Ok(_) => Ok(to_return),
                    Err(er) => Err(er)
                }
            }
            _ => Err(JqlError::UnknownQuery)
        };
    }

    fn is_collection_ok(cmd: &JqlCommand) -> Result<(), JqlError> {
        match is_naming_ok(&cmd._database) {
            None => Err(JqlError::NoDatabaseProvided),
            Some(_) => match is_naming_ok(&cmd._collection) {
                None => Err(JqlError::NoCollectionProvided),
                Some(_) => Ok(())
            }
        }
    }

    fn is_db_ok(cmd: &JqlCommand) -> Result<(), JqlError> {
        match is_naming_ok(&cmd._database) {
            None => Err(JqlError::NoDatabaseProvided),
            Some(_) => Ok(())
        }
    }

    fn is_create_collection_ok(cmd: &JqlCommand) -> Result<(), JqlError> {
        match is_naming_ok(&cmd._database) {
            None => Err(JqlError::NoDatabaseProvided),
            Some(_) => match cmd._body {
                None => Err(JqlError::NoSchemaProvided),
                Some(_) => {
                    let jqls = Jql::new_schema_parser();
                    let chk = jqls.parse_schema_with_json(Ok(cmd._body.clone().unwrap()));
                    if chk.is_ok() {
                        Ok(())
                    } else {
                        Err(chk.err().unwrap())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn create_collection() {
        let json = r#"
            {
                "_action":"create-collection",
                "_database":"moss",
                "_body":{
                    "_name":"person",
                    "_key":"ids",
                    "_fields":[
                        {
                            "_name":"hello",
                            "_declare":"string[]"
                        },
                        {
                            "_name":"hello1",
                            "_declare":"string[]"
                        }
                    ],
                    "_in_memory": false
                }
            }
        "#;
        let elp = Instant::now();
        let parser = Jql::parse(json);
        // println!("create_collection :: {:?}",elp.elapsed());
        assert!(parser.is_ok(), "{}", parser.err().unwrap().to_string());
    }

    #[test]
    fn create_db() {
        let json = r#"
            {
                "_action":"create-db",
                "_database":"moss",
                "_body":{ }
            }
        "#;
        let elp = Instant::now();
        let parser = Jql::parse(json);
        // println!("create_db :: {:?}",elp.elapsed());
        assert!(parser.is_ok());
    }

    #[test]
    fn drop_db() {
        let json = r#"
            {
                "_action":"drop-db",
                "_database":"moss",
                "_body":{ }
            }
        "#;
        let elp = Instant::now();
        let parser = Jql::parse(json);
        // println!("drop_db :: {:?}",elp.elapsed());
        assert!(parser.is_ok());
    }

    #[test]
    fn drop_collection() {
        let json = r#"
            {
                "_action":"drop-collection",
                "_database":"moss",
                "_collection":"ops"
            }
        "#;
        let elp = Instant::now();
        let parser = Jql::parse(json);
        // println!("drop_collection :: {:?}",elp.elapsed());
        assert!(parser.is_ok());
    }
}