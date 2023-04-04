use std::path::Path;
use std::{fs, thread};
use itertools::Itertools;
use rocksdb::{DB, DBCompactionStyle, DBCompressionType, DBRecoveryMode, Options};

pub fn use_available_threads() -> usize {
    thread::available_parallelism().map_or(1, usize::from) * 4
}

pub fn get_cfs(opts: &Options, path: &str) -> Vec<String> {
    DB::list_cf(opts, path)
        .unwrap_or(vec![])
        .into_iter()
        .filter(|cf| !cf.eq("default"))
        .clone()
        .collect()
}

pub fn cfg_db(log: &str) -> Options {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    opts.increase_parallelism(use_available_threads() as i32);
    opts.set_max_background_jobs(4);
    opts.set_bytes_per_sync(1048576);
    opts.set_db_log_dir(log);
    opts.set_compaction_style(DBCompactionStyle::Universal);
    opts.set_compression_type(DBCompressionType::Snappy);
    opts.set_use_fsync(false);
    opts.set_allow_concurrent_memtable_write(true);
    opts.set_allow_mmap_reads(true);
    opts.set_allow_mmap_writes(true);
    opts.set_optimize_filters_for_hits(true);
    opts.set_wal_recovery_mode(DBRecoveryMode::TolerateCorruptedTailRecords);
    opts
}

pub fn query_log_cf_path(log_path: &str) -> String {
    let path = Path::new(log_path);
    let log = path.join("query-log");
    format!("{}",log.to_str().unwrap())
}

pub fn key_merger(keys: Vec<String>) -> Option<String> {
    if keys.len() == 0 {
        return None;
    }
    Some(keys.join("#"))
}

pub fn key_splitter(key: String) -> Option<Vec<String>> {
    if !key.contains("#") {
        return None;
    }
    let keys = key.split("#").into_iter().map(|s|s.to_string()).collect_vec();
    Some(keys)
}

pub fn ksm_db(key: String) -> Option<String> {
    let keys = key_splitter(key);
    if keys.is_none() {
        return None;
    }
    let res = keys.unwrap();
    if res.len() != 2 {
        return None;
    }
    let k = res.get(1).unwrap().clone();
    Some(k)
}
