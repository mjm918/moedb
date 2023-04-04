use std::sync::Arc;
use log::{trace};
use serde_json::Value;
use crate::env;
use crate::err::{MoeDbError};
use crate::hdrs::{ActionType, Exec, Jql, Trx};

impl Exec {
    pub fn new(cfg: Arc<env::MoeDb>) -> Result<Self, MoeDbError> {
        let env = Arc::clone(&cfg);
        let trx = Trx::new(Arc::clone(&env), false);
        if trx.is_err() {
            return Err(trx.err().unwrap());
        }
        Ok(Self {
            env: cfg,
            trx: Arc::new(trx.unwrap())
        })
    }

    pub fn run(&self, id: &str, stmt: &str) -> Result<Option<Vec<Value>>, MoeDbError> {
        let cmd = stmt.to_string();
        let jql = Jql::parse(cmd.as_str());
        trace!("trying to parse . is error? {} ",jql.is_err());
        if jql.is_err() {
            let err = MoeDbError::QueryError(jql.err().unwrap().to_string());
            trace!("error on jql {}",err);
            return Err(err);
        }

        let parsed = jql.unwrap();
        trace!("parsed {:?}",parsed);

        let exec_res = match ActionType::from(parsed._action.as_str()) {
            ActionType::Create => self.new_col(&parsed),
            ActionType::CreateDb => self.new_db(&parsed),
            ActionType::Get => todo!(),
            ActionType::Upsert => todo!(),
            ActionType::Delete => todo!(),
            ActionType::Drop => self.drop_col(&parsed),
            ActionType::DropDb => self.drop_db(&parsed),
            ActionType::DbList => self.db_list(),
            ActionType::Truncate => self.trun_col(&parsed),
            ActionType::ColList => self.col_list(&parsed),
            _ => self.db_list()
        };

        if exec_res.error.is_some() {
            let err = MoeDbError::TransactionError(exec_res.error.unwrap().to_string());
            trace!("error on trx {}",err);
            return Err(err);
        }
        Ok(exec_res.data)
    }
}