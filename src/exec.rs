use anyhow::{Result};
use std::sync::Arc;
use std::time::Instant;
use rocksdb::Options;
use crate::headers::{Exec, MoeDbMode, ParsedStatement, Response, ActionType};

impl Exec {
    pub fn new(db: Arc<MoeDbMode>,opts: Arc<Options>) -> Self {
        Self {
            db,
            opts
        }
    }

    pub fn jql(stmt: String) -> Response {
        let bench = Instant::now();

        Response{
            time_taken: format!("{:?}",bench.elapsed()),
            error: false,
            message: "".to_string(),
            data: None,
        }
    }
}