use std::sync::Arc;
use std::sync::mpsc::channel;
use rayon::ThreadPoolBuilder;
use rocksdb::Options;
use tracing::trace;
use crate::env::env;
use crate::error::MoeDbError;
use crate::headers::{ActionType, Exec, Jql, JqlCommand, MoeDbMode, Trx};
use crate::util::use_available_threads;

impl Exec {
    pub fn new(db: Arc<MoeDbMode>) -> Self {
        let pool = ThreadPoolBuilder::new()
            .num_threads(use_available_threads())
            .build()
            .unwrap();
        Self {
            db,
            pool,
            env: Arc::new(env().unwrap())
        }
    }

    pub fn run(&self, stmt: &str) -> Result<(), MoeDbError> {
        let cmd = stmt.to_string();
        let (tx, rx) = channel::<Result<(), MoeDbError>>();
        let trx = Trx::new(Arc::clone(&self.db),Arc::clone(&self.env));
        self.pool.spawn(move ||{
            let jql = Jql::parse(cmd.as_str());
            trace!("trying to parse . is error? {} ",jql.is_err());
            if jql.is_err() {
                let err = MoeDbError::QueryError(jql.err().unwrap().to_string());
                trace!("error on jql {}",err);
                tx.send(Err(err)).unwrap();
                return;
            }

            let parsed = jql.unwrap();
            let exec_res = match ActionType::from(parsed._action) {
                ActionType::Create => todo!(),
                ActionType::CreateDb => trx.create_db(parsed._database.unwrap().as_str()),
                ActionType::Get => todo!(),
                ActionType::Upsert => todo!(),
                ActionType::Delete => todo!(),
                ActionType::Drop => todo!(),
                ActionType::DropDb => todo!(),
                ActionType::DbList => todo!(),
                ActionType::Truncate => todo!(),
            };

            if exec_res.is_err() {
                let err = MoeDbError::TransactionError(exec_res.err().unwrap().to_string());
                trace!("error on trx {}",err);
                tx.send(Err(err)).unwrap();
                return;
            }
            tx.send(Ok(())).unwrap();
        });
        let received = rx.recv().unwrap();
        if received.is_err() {
            return Err(received.err().unwrap());
        }
        Ok(())
    }
}