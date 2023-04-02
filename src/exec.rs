use std::sync::Arc;
use std::sync::mpsc::channel;
use rayon::ThreadPoolBuilder;
use rocksdb::Options;
use log::{trace};
use serde_json::Value;
use crate::env;
use crate::env::env;
use crate::error::{MoeDbError, TrxError};
use crate::headers::{ActionType, Exec, Jql, JqlCommand, MoeDbMode, Trx};
use crate::util::use_available_threads;

impl Exec {
    pub fn new(cfg: Arc<env::MoeDb>) -> Result<Self, MoeDbError> {
        let pool = ThreadPoolBuilder::new()
            .num_threads(use_available_threads())
            .build()
            .unwrap();
        let env = Arc::clone(&cfg);
        let trx = Trx::new(Arc::clone(&env));
        if trx.is_err() {
            return Err(trx.err().unwrap());
        }
        Ok(Self {
            pool,
            env: cfg,
            trx: Arc::new(trx.unwrap())
        })
    }

    pub fn run(&self, stmt: &str) -> Result<Option<Vec<Value>>, MoeDbError> {
        let cmd = stmt.to_string();
        let (tx, rx) = channel::<Result<Option<Vec<Value>>, MoeDbError>>();
        self.pool.scope(|scp|{
            scp.spawn(move |_|{
                let jql = Jql::parse(cmd.as_str());
                trace!("trying to parse . is error? {} ",jql.is_err());
                if jql.is_err() {
                    let err = MoeDbError::QueryError(jql.err().unwrap().to_string());
                    trace!("error on jql {}",err);
                    tx.send(Err(err)).unwrap();
                    return;
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
                    tx.send(Err(err)).unwrap();
                    return;
                }
                tx.send(Ok(exec_res.data)).unwrap();
            });
        });
        let received = rx.recv().unwrap();
        if received.is_err() {
            return Err(received.err().unwrap());
        }
        Ok(received.unwrap())
    }
}