use log::{trace, warn};
use crate::error::TrxError;
use crate::headers::{Exec, JqlCommand, Types};
use crate::util::key_merger;
use crate::var::{DB_PREFIX, DB_SYS};

impl Exec {
    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        false
    }

    pub fn new_db(&self, cmd: &JqlCommand) -> Result<(), TrxError> {
        let wrp_db = cmd._database.as_ref().unwrap();
        let db = wrp_db.as_str();
        let wrp_key = key_merger(vec![DB_PREFIX.to_string(), db.to_string()]);
        if wrp_key.is_some() {
            let key = wrp_key.unwrap();
            let exi = self.trx.get(DB_SYS, key.as_bytes().to_vec());
            return if exi.is_some() {
                Err(TrxError::CreateDbError(format!("db already exists `{}`", db)))
            } else {
                let created = self.trx.put(
                    DB_SYS,
                    key.as_bytes().to_vec(),
                    serde_json::to_string::<Vec<String>>(&vec![]).unwrap().as_bytes().to_vec()
                );
                if created.is_ok() {
                    Ok(())
                } else {
                    Err(TrxError::CreateDbError(created.err().unwrap().to_string()))
                }
            }
        }
        Err(TrxError::CreateCollectionError("check database name".to_string()))
    }

    pub fn new_col(&self, cmd: &JqlCommand) -> Result<(), TrxError> {
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
                Err(TrxError::CreateCollectionError(format!("collection already exists `{}`", col)))
            } else {
                let cf_created = self.trx.create_cf(key.as_str());
                return if cf_created.is_ok() {
                    let created = self.trx.put(
                        DB_SYS,
                        key.as_bytes().to_vec(),
                        serde_json::to_string(wrp_col).unwrap().as_bytes().to_vec()
                    );
                    if created.is_ok() {
                        Ok(())
                    } else {
                        Err(TrxError::CreateCollectionError(created.err().unwrap().to_string()))
                    }
                } else {
                    Err(TrxError::CreateCollectionError(cf_created.err().unwrap().to_string()))
                }
            }
        }
        Err(TrxError::CreateCollectionError("check database & collection name".to_string()))
    }

    pub fn trun_col(&self, cmd: &JqlCommand) -> Result<(), TrxError> {
        let wrp_key = Self::db_col_merged(&cmd);
        if wrp_key.is_some() {
            let key = wrp_key.unwrap();
            if !self.trx.cfs().contains(&key) {
                return Ok(());
            }
            let dropped = self.trx.truncate(key.as_str());
            return if dropped.is_ok() {
                Ok(())
            } else {
                Err(TrxError::TruncateCollectionError(dropped.err().unwrap().to_string()))
            }
        }
        Err(TrxError::TruncateCollectionError("check database & collection name".to_string()))
    }

    pub fn drop_col(&self, cmd: &JqlCommand) -> Result<(), TrxError> {
        let wrp_key = Self::db_col_merged(&cmd);
        if wrp_key.is_none() {
            return Err(TrxError::TruncateCollectionError("check database & collection name".to_string()));
        }

        let unwrp_key = wrp_key.unwrap();
        let key = unwrp_key.as_str().as_bytes().to_vec();
        let exi = self.trx.get(DB_SYS, key.clone());
        if exi.is_none() {
            return Err(TrxError::DropCollectionError("collection does not exist".to_string()));
        }
        let dropped = self.trx.cfs().contains(&unwrp_key);
        if dropped {
            let dlt = self.trx.delete(DB_SYS,key.clone());
            return if dlt.is_err() {
                Err(TrxError::DropCollectionError(dlt.err().unwrap().to_string()))
            } else {
                Ok(())
            }
        }
        let truncated = self.trun_col(&cmd);
        if truncated.is_ok() {
            let dlt = self.trx.delete(DB_SYS,key.clone());
            return if dlt.is_err() {
                Err(TrxError::DropCollectionError(dlt.err().unwrap().to_string()))
            } else {
                Ok(())
            }
        }
        Err(TrxError::UnknownError)
    }

    pub fn drop_db(&self, cmd: &JqlCommand) -> Result<(), TrxError> {
        let cfs = self.trx.cfs();
    }

    fn db_col_merged(cmd: &JqlCommand) -> Option<String> {
        let wrp_db = cmd._database.as_ref().unwrap();
        let wrp_col = cmd._collection.as_ref().unwrap();

        let db = wrp_db.as_str();
        let col = wrp_col.as_str();
        key_merger(vec![db.to_string(), col.to_string()])
    }
}