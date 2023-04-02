use std::str;
use std::collections::BTreeMap;
use std::sync::Arc;
use log::{error, trace};
use rocksdb::{BoundColumnFamily, DB, DBCommon, DBRawIterator, DBRawIteratorWithThreadMode, Direction, Error, IteratorMode, MultiThreaded, Options, PrefixRange, ReadOptions};
use serde_json::Value;
use crate::env;
use crate::error::{MoeDbError, TrxError};
use crate::headers::{JqlCommand, MoeDbMode, TKey, Trx, TValue};
use crate::util::{cfg_db, get_cfs};
use crate::var::{DB_CREDS, DB_LOG, DB_SYS};

impl Trx {

    pub fn new(env: Arc<env::MoeDb>) -> Result<Self, MoeDbError> {
        let ins = Self::init(env.db_path.as_str(), env.log_path.as_str());
        if ins.is_err() {
            return Err(ins.err().unwrap());
        }
        let tpl = ins.unwrap();
        let db = Arc::new(tpl.0);
        Ok(Self {
            db,
            env,
        })
    }

    fn init(path: &str, log: &str) -> Result<(MoeDbMode, Options), MoeDbError> {
        let (opts, cfs) = Self::opts_cf_pair(path, log);
        trace!("{}",format!("moedb cfs {:?}",&cfs));

        let ins = DB::open_cf(
            &opts,
            path,
            cfs.clone()
        );
        if ins.is_err() {
            return Err(MoeDbError::CfError(ins.err().unwrap().to_string()));
        }
        let moedb = ins.unwrap();

        vec![DB_CREDS, DB_SYS, DB_LOG]
            .into_iter()
            .for_each(|sys_db| {
                if !cfs.contains(&sys_db.to_string()) {
                    let cf_created = moedb.create_cf(sys_db, &opts);
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

    pub fn cfs(&self) -> Vec<String> {
        Self::opts_cf_pair(self.env.db_path.as_str(), self.env.log_path.as_str()).1
    }

    pub fn cf(&self, name: &str) -> Arc<BoundColumnFamily> {
        self.db.cf_handle(name).unwrap()
    }

    pub fn raw_iter(&self, name: &str) -> DBRawIterator {
        self.db.raw_iterator_cf(&self.cf(name))
    }

    pub fn range_iter(&self, name: &str, prefix: TKey) -> DBRawIterator {
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(PrefixRange(prefix.as_slice()));
        self.db.raw_iterator_cf_opt(&self.cf(name),opts)
    }

    pub fn create_cf(&self, name: &str) -> Result<(), TrxError> {
        let res = self.db.create_cf(name, &cfg_db(self.env.log_path.as_str()));
        if res.is_err() {
            return Err(TrxError::CreateDbError(res.err().unwrap().to_string()));
        }
        Ok(())
    }

    pub fn get(&self,cf_name: &str, key: TKey) -> Option<Value> {
        let cf = self.cf(cf_name);
        let v = self.db.get_cf(&cf, key);
        if v.is_ok() {
            let r = v.unwrap();
            if r.is_none() {
                return None;
            }
            return Some(serde_json::from_slice(&*r.unwrap()).unwrap())
        }
        None
    }

    pub fn put(&self, cf_name: &str, key: TKey, value: TValue) -> Result<(), Error> {
        let cf = self.cf(cf_name);
        self.db.put_cf(&cf, key, value)
    }

    pub fn delete(&self, cf_name: &str, key: TKey) -> Result<(), Error> {
        let cf = self.cf(cf_name);
        self.db.delete_cf(&cf, key)
    }

    pub fn truncate(&self, cf_name: &str) -> Result<(), Error> {
        self.db.drop_cf(cf_name)
    }

    pub fn through(&self, cf_name: &str, prefix: TKey) -> BTreeMap<String, Value> {
        let mut res = BTreeMap::new();
        let mut iter = self.range_iter(cf_name, prefix);
        iter.seek_to_first();
        while iter.valid() {
            let kv = iter.item().unwrap().clone();
            res.insert(str::from_utf8(kv.0).unwrap().to_string(),serde_json::from_slice::<Value>(kv.1).unwrap());
            iter.next();
        }
        res
        /*self.db.snapshot();
        let mut res = BTreeMap::new();
        let mut iter = self.raw_iter(cf_name);
        iter.seek(key);
        while iter.valid() {
            let kv = iter.item().unwrap().clone();
            res.insert(str::from_utf8(kv.0).unwrap().to_string(),serde_json::from_slice::<Value>(kv.1).unwrap());
            iter.next();
        }
        res*/
    }
}