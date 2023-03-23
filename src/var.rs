pub const ENV_PATH: &str = if cfg!(debug_assertions) {
    "/Users/julfikar/Documents/Personal.nosync/moedb/tests/moedb.toml"
} else {
    "./moedb.toml"
};

pub const DB_CREDS: &str = "::credentials::";
pub const DB_SYS: &str = "::sys::";
pub const DB_LOG: &str = "::logs::";
