mod error;
mod env;
mod var;
mod headers;
mod moedb;
mod exec;
mod doc;
mod util;
mod jqlv;
mod traits;
mod ops;
mod jqls;
mod dtp;
mod mtp;
mod jql;
mod act;
mod ctp;

#[cfg(test)]
mod tests {
    use crate::headers::MoeDb;
    use super::*;
    #[test]
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
        let db = MoeDb::new(cfg.db_path.as_str(), cfg.log_path.as_str());
        assert_eq!(db.is_err(),false,"{}",db.err().unwrap().to_string());
    }
}