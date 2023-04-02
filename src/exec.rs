use std::sync::Arc;
use std::sync::mpsc::channel;
use rayon::ThreadPoolBuilder;
use rocksdb::Options;
use log::{trace};
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

    pub fn run(&self, stmt: &str) -> Result<(), MoeDbError> {
        let cmd = stmt.to_string();
        let (tx, rx) = channel::<Result<(), MoeDbError>>();
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
                    ActionType::DropDb => todo!(),
                    ActionType::DbList => todo!(),
                    ActionType::Truncate => self.trun_col(&parsed),
                    _ => Err(TrxError::CreateDbError("unknown operation requested".to_string()))
                };

                if exec_res.is_err() {
                    let err = MoeDbError::TransactionError(exec_res.err().unwrap().to_string());
                    trace!("error on trx {}",err);
                    tx.send(Err(err)).unwrap();
                    return;
                }
                tx.send(Ok(())).unwrap();
            });
        });
        let received = rx.recv().unwrap();
        if received.is_err() {
            return Err(received.err().unwrap());
        }
        Ok(())
    }
}