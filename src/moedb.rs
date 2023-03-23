use anyhow::{Result};
use std::sync::Arc;
use rocksdb::{DB, Options};
use toml::Value;
use tracing::{error, Level, trace};
use crate::error::MoeDbError;
use crate::headers::{MoeDb, MoeDbMode};
use crate::util::cfg_db;
use crate::var::{DB_CREDS, DB_LOG, DB_SYS};

/// ---------------------------------------------
/// Start MoeDb
/// It will hold rocksdb instance
/// It will create `sys` db on boot
/// It will create `log` db on boot
/// It will initiate `tracing`
///
/// It will receive `mstmt` -- moedb statement
/// It will pass the `mstmt` to `engine`
/// Upon result from `engine`, it will return result to "caller"
/// ---------------------------------------------

impl<'a> MoeDb<'a> {
    pub fn new(path: &'a str, log: &'a str) -> Result<Self, MoeDbError> {
        let logger = tracing_appender::rolling::minutely(log, "[MoeDb]DEBUG");
        let (writer, _guard) = tracing_appender::non_blocking(logger);
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_writer(writer)
            .init();
        trace!("creating [moedb] instance");
        let ins = Self::init(path, log);
        if ins.is_err() {
            return Err(ins.err().unwrap());
        }
        let tpl = ins.unwrap();
        trace!("created [moedb] instance and system databases");
        Ok(Self {
            db: Arc::new(tpl.0),
            opts: Arc::new(tpl.1),
            path,
            log,
        })
    }

    pub async fn execute(&self, stmt: String) -> Vec<Value> {
        todo!()
    }

    fn init(path: &str, log: &str) -> Result<(MoeDbMode, Options), MoeDbError> {
        let opts = cfg_db(log);
        let cfs = Self::get_cfs(&opts, path);
        trace!("{}",format!("[moedb] cfs {:?}",&cfs));
        let ins = DB::open_cf(&opts, path, cfs.clone());
        if ins.is_err() {
            return Err(MoeDbError::CfError(ins.err().unwrap().to_string()));
        }
        let moedb = ins.unwrap();
        let db = Arc::new(&moedb);
        vec![DB_CREDS, DB_SYS, DB_LOG]
            .into_iter()
            .for_each(|sys_db|{
                if !cfs.contains(&sys_db.to_string()) {
                    let cf_created = db.create_cf(sys_db, &opts);
                    if cf_created.is_err() {
                        error!("{}",MoeDbError::CfCreateError(cf_created.err().unwrap().to_string()));
                    }
                }
            });
        Ok((moedb, opts))
    }

    fn get_cfs(opts: &Options, path: &str) -> Vec<String> {
        DB::list_cf(opts, path)
            .unwrap_or(vec![])
            .into_iter()
            .filter(|cf| !cf.eq("default"))
            .clone()
            .collect()
    }
}