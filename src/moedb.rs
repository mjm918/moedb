use std::io::stdout;
use anyhow::{Result};
use std::sync::Arc;
use std::time::Instant;
use rocksdb::{DB, Options};
use toml::Value;
use tracing::{error, info, instrument, Level, trace};
use crate::env;
use crate::error::MoeDbError;
use crate::func::unique_id;
use crate::headers::{Exec, Jql, MoeDb, MoeDbMode, Response};
use crate::util::{cfg_db, get_cfs};
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

impl MoeDb {
    pub fn new(path: String, log: String) -> Result<Self, MoeDbError> {
        let logger = tracing_appender::rolling::minutely(log.as_str(), "[MoeDb]DEBUG");
        let (writer, _guard) = tracing_appender::non_blocking(logger);
        let (console, _guard) = tracing_appender::non_blocking(stdout());
        if cfg!(debug_assertions) {
            tracing_subscriber::fmt()
                .with_max_level(Level::TRACE)
                .with_writer(console)
                .with_target(false)
                .with_line_number(true)
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_max_level(Level::TRACE)
                .with_writer(writer)
                .init();
        }

        trace!("creating moedb instance");
        let ins = Self::init(path.as_str(), log.as_str());
        if ins.is_err() {
            return Err(ins.err().unwrap());
        }
        let tpl = ins.unwrap();
        let db = Arc::new(tpl.0);

        trace!("created moedb instance and system databases");
        Ok(Self {
            db: db.clone(),
            exec: Exec::new(Arc::clone(&db))
        })
    }
    #[instrument(ret)]
    pub fn execute(&self, stmt: String) -> Response {
        let elp = Instant::now();

        let query_id = unique_id();
        trace!("received a new query with given ID");

        let res = self.exec.run(stmt.as_str());
        let ended = format!("{:?}",elp.elapsed());

        trace!("executed query");

        Response {
            time_taken: ended,
            error: res.is_err(),
            message: res.err().unwrap().to_string(),
            data: None,
        }
    }

    fn init(path: &str, log: &str) -> Result<(MoeDbMode, Options), MoeDbError> {
        let (opts, cfs) = Self::opts_cf_pair(path, log);
        trace!("{}",format!("moedb cfs {:?}",&cfs));

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

    fn opts_cf_pair(path: &str, log: &str) -> (Options, Vec<String>) {
        let opts = cfg_db(log);
        let ref_opts = opts.clone();
        (opts, get_cfs(&ref_opts, path))
    }
}