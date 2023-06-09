use std::fs::File;
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TerminalMode, TermLogger, WriteLogger};
use crate::err::MoeDbError;
use crate::func::unique_id;
use crate::hdrs::MoeDb;

mod err;
mod env;
mod var;
mod hdrs;
mod util;
mod jqlv;
mod jqls;
mod dtp;
mod mtp;
mod jql;
mod act;
mod ctp;
mod func;
mod trx;
mod exec;
mod moedb;
mod doc;
mod resp;
mod sys;
mod log;

fn start_moedb() -> Result<MoeDb, MoeDbError> {
    let cfg = env::env().unwrap();
    let log_file = format!("{}/{}.LOG",cfg.log_path.as_str(),unique_id());
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create(log_file.as_str()).unwrap()),
        ]
    ).unwrap();

    MoeDb::new(cfg)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::hdrs::MoeDb;
    use super::*;
    #[test]
    pub fn open_env() {
        let cfg = env::env();
        assert_eq!(cfg.is_err(), false, "no error reading env file");
    }

    #[test]
    pub fn read_moedb_path() {
        let cfg = env::env();
        assert_eq!(cfg.unwrap().db_path.as_str().is_empty(), false, "moedb path not empty");
    }

    #[test]
    pub fn create_db() {
        let elp = Instant::now();
        let db = start_moedb().unwrap();
        let res = db.execute(r#"
            {
                "_action":"create-db",
                "_database":"random",
                "_body":{}
            }
        "#);
        let pr = res.clone();
        println!("create_db {:?} response {} res.error {} message {}", elp.elapsed(), pr, res.error, res.message);

        create_collection(db);
    }

    pub fn create_collection(db: MoeDb) {
        let elp = Instant::now();
        let res = db.execute(r#"
            {
                "_action":"create-collection",
                "_database":"random",
                "_body":{
                    "_name":"numbers",
                    "_key":"ids",
                    "_fields":[
                        {
                            "_name":"hello",
                            "_declare":"string[]"
                        }
                    ],
                    "_in_memory": false
                }
            }
        "#);
        let pr = res.clone();
        println!("create_collection {:?} response {} res.error {} message {}", elp.elapsed(), pr, res.error, res.message);

        db_list(db);
    }

    pub fn db_list(db: MoeDb) {
        let elp = Instant::now();
        let res = db.execute(r#"
            {
                "_action":"db-*"
            }
        "#);
        let pr = res.clone();
        assert!(!res.error, "{}", res.message);
        println!("db_list {:?} response {} res.error {}", elp.elapsed(), serde_json::to_string(&pr.data.unwrap()).unwrap(), res.error);

        col_list(db);
    }

    pub fn col_list(db: MoeDb) {
        let elp = Instant::now();
        let res = db.execute(r#"
            {
                "_action":"col-*",
                "_database":"random"
            }
        "#);
        let pr = res.clone();
        assert!(!res.error, "{}", res.message);
        println!("col_list {:?} response {} res.error {}", elp.elapsed(), serde_json::to_string(&pr.data.unwrap()).unwrap(), res.error);

        truncate_collection(db);
    }

    pub fn truncate_collection(db: MoeDb) {
        let elp = Instant::now();
        let res = db.execute(r#"
            {
                "_action":"truncate",
                "_database":"random",
                "_collection":"numbers"
            }
        "#);
        let pr = res.clone();
        println!("truncate_collection {:?} response {} res.error {} message {}", elp.elapsed(), pr, res.error, res.message);

        drop_collection(db);
    }

    pub fn drop_collection(db: MoeDb) {
        let elp = Instant::now();
        let res = db.execute(r#"
            {
                "_action":"drop-collection",
                "_database":"random",
                "_collection":"numbers"
            }
        "#);
        let pr = res.clone();
        assert!(!res.error, "{}", res.message);
        println!("drop_collection {:?} response {} res.error {}", elp.elapsed(), pr, res.error);

        drop_db(db);
    }

    pub fn drop_db(db: MoeDb) {
        let elp = Instant::now();
        let res = db.execute(r#"
            {
                "_action":"drop-db",
                "_database":"random"
            }
        "#);
        let pr = res.clone();
        assert!(!res.error, "{}", res.message);
        println!("drop_db {:?} response {} res.error {}", elp.elapsed(), pr, res.error);
    }
}