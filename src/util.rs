use std::thread;
use rocksdb::{DB, DBCompactionStyle, DBCompressionType, DBRecoveryMode, Options};

pub fn use_available_threads() -> usize {
    (thread::available_parallelism().map_or(1, usize::from) * 4)
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
    opts.set_max_open_files(-1);
    opts.set_max_background_jobs(4);
    opts.set_bytes_per_sync(1048576);
    opts.set_db_log_dir(log);
    opts.set_disable_auto_compactions(false);
    opts.set_compaction_style(DBCompactionStyle::Universal);
    opts.set_compression_type(DBCompressionType::Snappy);
    opts.set_use_fsync(false);
    opts.set_level_zero_file_num_compaction_trigger(4);
    opts.set_level_zero_stop_writes_trigger(12);
    opts.set_level_zero_slowdown_writes_trigger(8);
    opts.set_allow_concurrent_memtable_write(true);
    opts.set_allow_mmap_reads(true);
    opts.set_allow_mmap_writes(true);
    opts.set_num_levels(6);
    opts.set_optimize_filters_for_hits(true);
    opts.set_wal_recovery_mode(DBRecoveryMode::TolerateCorruptedTailRecords);
    opts.set_table_cache_num_shard_bits(6);
    opts
}
