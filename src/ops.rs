use anyhow::Result;
use std::sync::Arc;
use rocksdb::Options;
use crate::error::OpsError;
use crate::headers::{MoeDbMode, Ops, ParsedStatement};

impl Ops {
    pub fn new(db: Arc<MoeDbMode>,opts: Arc<Options>) -> Self {
        Self {
            db,
            opts
        }
    }

    pub fn create_db(&self,stmt: ParsedStatement) -> Result<(), OpsError> {
        let res = self.db.create_cf(stmt.db.as_str(),&self.opts);
        if res.is_err() {
            return Err(OpsError::CreateDbError(res.err().unwrap().to_string()));
        }
        Ok(())
    }

}