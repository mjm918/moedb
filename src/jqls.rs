use std::fmt::Debug;
use anyhow::{Result};
use fancy_regex::Regex;
use serde_json::Value;
use toml::value::Array;
use tracing::error;
use valico::json_dsl;
use valico::json_dsl::{array, array_of, boolean, Builder, Coercer, Param, string};
use crate::error::JqlError;
use crate::headers::{DataTypes, Jql, Types};
use crate::traits::JqlSchemaParser;
///
/// Schema Checker
///
impl JqlSchemaParser for Jql {
    fn new() -> Self {
        Self {
            prs: Builder::build(|b|{
                b.req(Types::Name.as_str(), Jql::naming_regx);
                b.req( Types::Key.as_str(),Jql::naming_regx);
                b.req(Types::Fields.as_str(),|p|{
                    p.coerce(array());
                    p.nest(|n|{
                        n.req(Types::Name.as_str(),Jql::naming_regx);
                        n.req(Types::Declare.as_str(),Jql::type_declaration);
                    });
                });
                b.opt_typed(Types::InMemory.as_str(),boolean());
            })
        }
    }

    fn parse(&self, values: String) -> Result<(), JqlError> {
        let mut val = serde_json::from_str(values.as_str());
        self.parse_with_value(val)
    }

    fn parse_with_value(&self, values: serde_json::Result<Value>) -> std::result::Result<(), JqlError> {
        if values.is_ok() {
            let mut v = values.unwrap();
            let state = self.prs.process(&mut v, None);
            return if state.is_strictly_valid() {
                let props = v.as_object().unwrap();
                for o in props {
                    if o.0.eq(Types::Fields.as_str()) {
                        let fields = o.1.as_array().unwrap();
                        if fields.len() == 0 {
                            let err = JqlError::NoFieldsProvided(v.to_string());
                            error!("{}",err);
                            return Err(err);
                        }
                    }
                }
                Ok(())
            } else {
                let err = JqlError::DocumentPropertyDataTypeError(format!("{:?}", state.errors));
                error!("{}",err);
                Err(err)
            }
        } else {
            let err = JqlError::MalformedSchema;
            error!("{}",err);
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    #[test]
    fn schema() {
        let schema = r#"
            {
                "_name":"person",
                "_key":"id",
                "_fields":[
                    {
                        "_name":"hello",
                        "_declare":"string[]"
                    }
                ],
                "_in_memory": false
            }
        "#;
        let jql = Jql::new();
        let elp = Instant::now();
        let res = jql.parse(schema.to_string());
        assert!(!res.is_err(),"{}",res.err().unwrap());
        println!("schema {:?}",elp.elapsed());
    }
}