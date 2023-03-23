use std::thread;
use rocksdb::{DB, DBCompactionStyle, DBCompressionType, Options};

pub fn use_available_threads() -> usize {
    (thread::available_parallelism().map_or(1, usize::from) * 4).next_power_of_two()
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
    opts.set_error_if_exists(false);
    opts.set_max_file_opening_threads(-1);
    opts.set_compression_type(DBCompressionType::Snappy);
    opts.set_db_log_dir(log);
    opts.set_compaction_style(DBCompactionStyle::Fifo);
    opts.set_use_fsync(true);
    opts.set_bytes_per_sync(8388608);
    opts.optimize_for_point_lookup(1024);
    opts.set_table_cache_num_shard_bits(6);
    opts.set_allow_concurrent_memtable_write(true);
    opts.set_allow_mmap_writes(true);
    opts.set_allow_mmap_reads(true);
    opts.set_optimize_filters_for_hits(true);
    opts
}
