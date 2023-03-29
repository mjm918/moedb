use crate::error::MoeDbError;
use crate::headers::MoeDb;

mod error;
mod env;
mod var;
mod headers;
mod util;
mod jqlv;
mod traits;
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

pub fn start_moedb() -> Result<MoeDb, MoeDbError> {
    let cfg = env::env().unwrap();
    MoeDb::new(cfg.db_path, cfg.log_path)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::headers::MoeDb;
    use super::*;
    /*#[test]
    pub fn open_env(){
        let cfg = env::env();
        assert_eq!(cfg.is_err(),false,"no error reading env file");
    }
    #[test]
    pub fn read_moedb_path(){
        let cfg = env::env();
        assert_eq!(cfg.unwrap().db_path.as_str().is_empty(),false,"moedb path not empty");
    }
    #[test]
    pub fn open_moedb(){
        let cfg = env::env().unwrap();
        let db = MoeDb::new(cfg.db_path, cfg.log_path);
        assert_eq!(db.is_err(),false,"{}",db.err().unwrap().to_string());
    }*/
    #[test]
    pub fn create_db() {
        let elp = Instant::now();
        let db = start_moedb();
        assert!(db.is_ok(),"db start");
        let moedb = db.unwrap();
        let res = moedb.execute(r#"
            {
                "_action":"create-db",
                "_database":"random",
                "_body":{}
            }
        "#.to_string());
        let pr = res.clone();
        assert!(res.error,"{}",res.message);
        println!("create_db {:?} response {}",elp.elapsed(), pr);
    }
}