use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub data_dir: PathBuf,
}

impl Config {
    /// Resolves the data directory: `--data-dir` flag > `MINI_DATA_DIR` env var > platform data dir > `./.mini`.
    pub fn resolve(data_dir_flag: Option<PathBuf>) -> Self {
        let data_dir = data_dir_flag
            .or_else(|| std::env::var_os("MINI_DATA_DIR").map(PathBuf::from))
            .or_else(|| dirs::data_dir().map(|d| d.join("mini")))
            .unwrap_or_else(|| PathBuf::from(".mini"));
        Config { data_dir }
    }
}
