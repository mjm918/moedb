use std::sync::Arc;
use log::{error, trace};
use crate::env;
use crate::err::MoeDbError;
use crate::hdrs::{Logging, Response, Trx};
use crate::util::key_merger;
use crate::var::{DB_LOG, QUERY_LOG_PREFIX, RES_LOG_PREFIX};

impl Logging {

    pub fn new(cfg: Arc<env::MoeDb>) -> Result<Self, MoeDbError> {
        let env = Arc::clone(&cfg);
        let trx = Trx::new(Arc::clone(&env), true);
        if trx.is_err() {
            return Err(trx.err().unwrap());
        }
        Ok(Self {
            trx: Arc::new(trx.unwrap())
        })
    }
}
