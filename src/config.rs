use clap::Clap;
use dirs_next::home_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Clap, Debug)]
pub struct Config {
    /// Root directory of the snm installation
    #[clap(long, name = "base-dir", env = "SNM_DIR", global = true)]
    pub snm_dir: Option<PathBuf>,

    /// Nodejs release mirror
    #[clap(
        long = "node-dist-mirror",
        name = "mirror",
        env = "SNM_NODE_DIST_MIRROR",
        default_value = "https://nodejs.org/dist",
        global = true
    )]
    pub dist_mirror: Url,

    /// Log level for the snm commands
    #[clap(
        long,
        name = "level",
        env = "SNM_LOGLEVEL",
        default_value = "info",
        global = true
    )]
    pub log_level: String,

    /// Only download the matching Nodejs version
    #[clap(short, long, global = true)]
    pub download_only: bool,
}

// Config::default() is intended to only be used within tests
impl Default for Config {
    fn default() -> Self {
        Config {
            download_only: false,
            log_level: "info".to_string(),
            snm_dir: home_dir().unwrap().join(".snm_test").into(),
            dist_mirror: Url::parse("https://nodejs.org/dist").unwrap(),
        }
    }
}

impl Config {
    pub fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    pub fn snm_home(&self) -> PathBuf {
        self.ensure_create(
            self.snm_dir
                .clone()
                .unwrap_or_else(|| home_dir().expect("Can't get home directory.").join(".snm")),
        )
    }

    pub fn release_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("releases"))
    }

    pub fn alias_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("aliases"))
    }

    pub fn alias_default(&self) -> PathBuf {
        self.alias_dir().join("default")
    }
}
