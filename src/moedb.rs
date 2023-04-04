use anyhow::{Result};
use std::sync::Arc;
use std::time::Instant;
use crossbeam::channel::unbounded;
use log::{trace};
use crate::env;
use crate::err::MoeDbError;
use crate::func::unique_id;
use crate::hdrs::{Exec, Logging, MoeDb, Response};

impl MoeDb {
    pub fn new(cfg: env::MoeDb) -> Result<Self, MoeDbError> {
        let config = Arc::new(cfg);
        let exec = Exec::new(Arc::clone(&config));
        if exec.is_err() {
            return Err(exec.err().unwrap());
        }
        let log = Logging::new(Arc::clone(&config));
        if log.is_err() {
            return Err(log.err().unwrap());
        }
        Ok(Self {
            exec: Arc::new(exec.unwrap()),
            log: Arc::new(log.unwrap())
        })
    }
    ///
    // create_db 27.280959ms response query executed in "4.23975ms"
    // create_collection 19.507875ms response query executed in "19.24775ms"
    // db_list 660.291µs response ["random"]
    // col_list 4.234375ms response ["numbers"]
    ///
    pub fn execute(&self, stmt: &str) -> Response {
        let elp = Instant::now();
        let uid = unique_id();
        let query_id = uid.as_str();
        trace!("received a new query with given ID {}", query_id.clone());

        let res = self.exec.run(query_id.clone(),stmt);
        let err = res.is_err();
        let mut message =  "".to_string();
        let mut data = None;
        if err {
            message = res.as_ref().err().unwrap().to_string();
        } else {
            data = res.unwrap();
        }
        let ended = format!("{:?}", elp.elapsed());
        let response = Response {
            time_taken: ended.clone(),
            error: err,
            message,
            data,
        };
        trace!("executed query {} in {}",query_id.clone(), ended.as_str());

        response
    }
}