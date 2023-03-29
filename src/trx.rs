use std::sync::Arc;
use rocksdb::Options;
use crate::env;
use crate::error::TrxError;
use crate::headers::{JqlCommand, MoeDbMode, TKey, Trx, TValue};
use crate::util::cfg_db;

impl Trx {
    pub fn new(db: Arc<MoeDbMode>, env: Arc<env::MoeDb>) -> Self {
        Self {
            db,
            env
        }
    }

    pub fn create_db(&self, name: &str) -> Result<(), TrxError> {
        let res = self.db.create_cf(name,&cfg_db(self.env.log_path.as_str()));
        if res.is_err() {
            return Err(TrxError::CreateDbError(res.err().unwrap().to_string()));
        }
        Ok(())
    }
}