use crate::log::LogLevel;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FrumConfig {
    pub base_dir: Option<PathBuf>,
    pub ruby_build_mirror: reqwest::Url,
    pub log_level: LogLevel,
    pub frum_path: Option<PathBuf>,
}

impl Default for FrumConfig {
    fn default() -> Self {
        Self {
            base_dir: std::env::var("FRUM_DIR").map(std::path::PathBuf::from).ok(),
            ruby_build_mirror: reqwest::Url::parse("https://cache.ruby-lang.org/pub/ruby").unwrap(),
            log_level: LogLevel::default(),
            frum_path: std::env::var("FRUM_MULTISHELL_PATH")
                .map(std::path::PathBuf::from)
                .ok(),
        }
    }
}

impl FrumConfig {
    pub fn base_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists((self.base_dir.clone()).unwrap_or_else(|| {
            dirs::home_dir()
                .expect("Can't get home directory")
                .join(".frum")
        }))
    }

    pub fn versions_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("versions"))
    }

    pub fn default_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("aliases"))
    }

    pub fn temp_installations_dir(&self) -> std::path::PathBuf {
        self.versions_dir().join(".downloads")
    }
}

fn ensure_dir_exists<T: AsRef<std::path::Path>>(path: T) -> T {
    std::fs::create_dir_all(path.as_ref()).ok();
    path
}
