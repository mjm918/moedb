use serde_json::Value;
use log::{error};
use valico::json_dsl::{array_of, boolean, Builder, f64, i64, string, u64};
use crate::error::JqlError;
use crate::headers::{DataTypes, Jql, JqlSchema};
use crate::traits::JqlValueParser;

///
/// Value checker
///
impl JqlValueParser for Jql {
    fn new_value_parser(base: String) -> Self {
        let schema: JqlSchema = serde_json::from_str(base.as_str()).unwrap();
        let mut builder = Builder::build(|f| {
            for pair in schema._fields {
                let nv = pair._name;
                let cr = match DataTypes::from(pair._declare.clone()) {
                    DataTypes::Int => i64(),
                    DataTypes::Uint => u64(),
                    DataTypes::Float => f64(),
                    DataTypes::String => string(),
                    DataTypes::Boolean => boolean(),
                    DataTypes::DateTime => string(),
                    DataTypes::Date => string(),
                    DataTypes::Time => string(),
                    DataTypes::ArrayOfString => array_of(string()),
                    DataTypes::ArrayOfInt => array_of(i64()),
                    DataTypes::ArrayOfUint => array_of(u64()),
                    DataTypes::ArrayOfFloat => array_of(f64()),
                    DataTypes::ArrayOfBoolean => array_of(boolean())
                };
                f.req(nv.as_str(), |p| {
                    p.coerce(cr);
                    if DataTypes::from(pair._declare.clone()) == DataTypes::DateTime {
                        Jql::declare_datetime(p);
                    }
                    if DataTypes::from(pair._declare.clone()) == DataTypes::Date {
                        Jql::declare_date(p);
                    }
                    if DataTypes::from(pair._declare.clone()) == DataTypes::Time {
                        Jql::declare_time(p);
                    }
                });
            }
        });
        Self {
            prs: builder
        }
    }

    fn parse_value(&self, mut values: String) -> Result<(), JqlError> {
        let mut val = serde_json::from_str(values.as_str());
        self.parse_value_with_json(val)
    }

    fn parse_value_with_json(&self, values: serde_json::error::Result<Value>) -> Result<(), JqlError> {
        if values.is_ok() {
            let mut v = values.unwrap();
            let state = self.prs.process(&mut v, None);
            return if state.is_strictly_valid() {
                Ok(())
            } else {
                let err = JqlError::DocumentPropertyDataTypeError(format!("{:?}", state));
                error!("{}",err);
                Err(err)
            };
        } else {
            let err = JqlError::DocumentParseError;
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
    fn val_check() {
        let schema = r#"
            {
                "_name":"person",
                "_key":"id",
                "_fields":[
                    {
                        "_name":"hello",
                        "_declare":"time"
                    }
                ],
                "_in_memory": false
            }
        "#;
        let mut value = String::from(r#"
            {
                "hello":"01:23:23"
            }
        "#);
        let elp = Instant::now();
        let jql = Jql::new_value_parser(schema.to_string());
        for i in 0..1 {
            let res = jql.parse_value(format!("[{}]", value.to_string()));
            assert!(res.is_ok(), "{}", res.err().unwrap());
        }
        // println!("schema {:?}",elp.elapsed());
    }
}