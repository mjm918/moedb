use std::io::stdout;
use anyhow::{Result};
use std::sync::Arc;
use std::time::Instant;
use rocksdb::{DB, Options};
use toml::Value;
use log::{trace, info, error};
use crate::env;
use crate::error::MoeDbError;
use crate::func::unique_id;
use crate::headers::{Exec, Jql, MoeDb, MoeDbMode, Response};
use crate::util::{cfg_db, get_cfs};
use crate::var::{DB_CREDS, DB_LOG, DB_SYS};

impl MoeDb {
    pub fn new(cfg: env::MoeDb) -> Result<Self, MoeDbError> {
        let exec = Exec::new(Arc::new(cfg));
        if exec.is_err() {
            return Err(exec.err().unwrap());
        }
        Ok(Self {
            exec: exec.unwrap()
        })
    }

    pub fn execute(&self, stmt: String) -> Response {
        let elp = Instant::now();

        let query_id = unique_id();
        trace!("received a new query with given ID {}", query_id.as_str());

        let res = self.exec.run(stmt.as_str());
        let ended = format!("{:?}", elp.elapsed());

        trace!("executed query {} in {}",query_id.as_str(), ended.as_str());
        let err = res.is_err();
        let mut message =  "".to_string();
        let mut data = None;
        if err {
            message = res.as_ref().err().unwrap().to_string();
        } else {
            data = res.unwrap();
        }
        Response {
            time_taken: ended,
            error: err,
            message,
            data,
        }
    }
}