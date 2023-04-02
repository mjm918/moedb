use itertools::Itertools;
use log::{error, info, trace, warn};
use crate::error::TrxError;
use crate::headers::{DbRes, Exec, JqlCommand, Types};
use crate::util::{key_merger, ksm_db};
use crate::var::{DB_PREFIX, DB_SYS};

impl Exec {
    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        false
    }

    pub fn new_db(&self, cmd: &JqlCommand) -> DbRes {
        let wrp_db = cmd._database.as_ref().unwrap();
        let db = wrp_db.as_str();
        let wrp_key = key_merger(vec![DB_PREFIX.to_string(), db.to_string()]);
        if wrp_key.is_some() {
            let key = wrp_key.unwrap();
            let exi = self.trx.get(DB_SYS, key.as_bytes().to_vec());
            return if exi.is_some() {
                DbRes { data: None, error: Some(TrxError::CreateDbError(format!("db already exists `{}`", db))) }
            } else {
                let created = self.trx.put(
                    DB_SYS,
                    key.as_bytes().to_vec(),
                    serde_json::to_string::<bool>(&true).unwrap().as_bytes().to_vec()
                );
                if created.is_ok() {
                    DbRes { data: None, error: None }
                } else {
                    DbRes { data: None, error: Some(TrxError::CreateDbError(created.err().unwrap().to_string())) }
                }
            }
        }
        DbRes { data: None, error: Some(TrxError::CreateCollectionError("check database name".to_string())) }
    }

    pub fn new_col(&self, cmd: &JqlCommand) -> DbRes {
        let wrp_db = cmd._database.as_ref().unwrap();
        let wrp_col = cmd._body.as_ref().unwrap();

        let db = wrp_db.as_str();
        let col = wrp_col
            .as_object()
            .unwrap()
            .get(Types::Name.as_str())
            .unwrap()
            .as_str()
            .unwrap();

        let wrp_key = key_merger(vec![db.to_string(), col.to_string()]);
        if wrp_key.is_some() {
            let key = wrp_key.unwrap();
            let exi = self.trx.get(DB_SYS,key.as_bytes().to_vec());

            return if exi.is_some() {
                DbRes { data: None, error: Some(TrxError::CreateCollectionError(format!("collection already exists `{}`", col))) }
            } else {
                let cf_created = self.trx.create_cf(key.as_str());
                return if cf_created.is_ok() {
                    let created = self.trx.put(
                        DB_SYS,
                        key.as_bytes().to_vec(),
                        serde_json::to_string(wrp_col).unwrap().as_bytes().to_vec()
                    );
                    if created.is_ok() {
                        DbRes { data: None, error: None }
                    } else {
                        DbRes { data: None, error: Some(TrxError::CreateCollectionError(created.err().unwrap().to_string())) }
                    }
                } else {
                    DbRes { data: None, error: Some(TrxError::CreateCollectionError(cf_created.err().unwrap().to_string())) }
                }
            }
        }
        DbRes { data: None, error: Some(TrxError::CreateCollectionError("check database & collection name".to_string())) }
    }

    pub fn trun_col(&self, cmd: &JqlCommand) -> DbRes {
        let wrp_key = Self::db_col_merged(&cmd);
        if wrp_key.is_some() {
            let key = wrp_key.unwrap();
            if !self.trx.cfs().contains(&key) {
                return DbRes { data: None, error: None };
            }
            let dropped = self.trx.truncate(key.as_str());
            return if dropped.is_ok() {
                DbRes { data: None, error: None }
            } else {
                DbRes { data: None, error: Some(TrxError::TruncateCollectionError(dropped.err().unwrap().to_string())) }
            }
        }
        DbRes { data: None, error: Some(TrxError::TruncateCollectionError("check database & collection name".to_string())) }
    }

    pub fn drop_col(&self, cmd: &JqlCommand) -> DbRes {
        let wrp_key = Self::db_col_merged(&cmd);
        if wrp_key.is_none() {
            return DbRes { data: None, error: Some(TrxError::TruncateCollectionError("check database & collection name".to_string())) };
        }
        let mut is_err = false;
        let unwrp_key = wrp_key.unwrap();
        let key = unwrp_key.as_str().as_bytes().to_vec();
        let exi = self.trx.get(DB_SYS, key.clone());
        if exi.is_none() {
            return DbRes { data: None, error: Some(TrxError::DropCollectionError("collection does not exist".to_string())) };
        }
        let mut dlt = Ok(());
        let dropped = self.trx.cfs().contains(&unwrp_key);
        if dropped {
            dlt = self.trx.delete(DB_SYS,key.clone());
            is_err = dlt.is_err();
        } else {
            let truncated = self.trun_col(&cmd);
            if truncated.error.is_none() {
                dlt = self.trx.delete(DB_SYS,key.clone());
                is_err = dlt.is_err();
            }
        }
        if is_err {
            DbRes { data: None, error: Some(TrxError::DropCollectionError(dlt.err().unwrap().to_string())) }
        } else {
            DbRes { data: None, error: None }
        }
    }

    pub fn drop_db(&self, cmd: &JqlCommand) -> DbRes {
        let mut dropped = false;
        let wrp_db = cmd._database.as_ref().unwrap();
        let wrp_prefix = key_merger(vec![DB_PREFIX.to_string(),wrp_db.clone().to_string()]).unwrap();
        let cfs = self.col_list(&cmd)
            .data
            .unwrap()
            .into_iter()
            .map(|p|p.as_str().unwrap().to_string())
            .collect_vec();

        if cfs.is_empty() {
            warn!("no cfs to drop {:?}",cmd);
            dropped = true;
        }

        for cf in cfs {
            let chk = self.trx.truncate(cf.as_str());
            if chk.is_err() {
                error!("drop_db {}",chk.err().unwrap());
            } else {
                let chk = self.trx.delete(DB_SYS, cf.as_str().as_bytes().to_vec());
                if chk.is_err() {
                    error!("delete from sys {}",chk.err().unwrap());
                }
                dropped = true;
            }
        }
        if dropped {
            let dlt = self.trx.delete(DB_SYS,wrp_prefix.as_bytes().to_vec());
            if dlt.is_err() {
                return DbRes { data: None, error: Some(TrxError::DropDbError(dlt.err().unwrap().to_string())) };
            }
        }
        DbRes { data: None, error: None }
    }

    pub fn db_list(&self) -> DbRes {
        let key_prefix = key_merger(vec![DB_PREFIX.to_string(),"".to_string()]).unwrap().as_bytes().to_vec();
        let dbs = self.trx.through(DB_SYS,key_prefix);
        let mut items = vec![];
        for db in dbs {
            info!("{} {:?}",db.0.as_str(),db.1);
            items.push(serde_json::to_value(ksm_db(db.0).unwrap()).unwrap());
        }
        DbRes { data: Some(items), error: None }
    }

    pub fn col_list(&self, cmd: &JqlCommand) -> DbRes {
        let wrp_db = cmd._database.as_ref().unwrap();
        let prefix = key_merger(vec![wrp_db.clone(),"".to_string()]).unwrap();
        let cfs = self.trx.cfs()
            .into_iter()
            .filter(|p|p.starts_with(&prefix))
            .map(|p|serde_json::to_value(ksm_db(p)).unwrap())
            .collect_vec();
        DbRes { data: Some(cfs), error: None }
    }

    fn db_col_merged(cmd: &JqlCommand) -> Option<String> {
        let wrp_db = cmd._database.as_ref().unwrap();
        let wrp_col = cmd._collection.as_ref().unwrap();

        let db = wrp_db.as_str();
        let col = wrp_col.as_str();
        key_merger(vec![db.to_string(), col.to_string()])
    }
}