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

    // Log level for the snm commands
    // #[clap(
    //     long,
    //     name = "level",
    //     env = "SNM_LOGLEVEL",
    //     default_value = "info",
    //     global = true
    // )]
    // pub log_level: String,
    //
    /// Only download the matching Nodejs version, don't use it
    #[clap(short, long, global = true)]
    pub no_use: bool,
}

impl Config {
    pub fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    pub fn snm_home(&self) -> PathBuf {
        self.snm_dir
            .to_owned()
            .unwrap_or_else(|| home_dir().expect("Can't get home directory.").join(".snm"))
    }

    pub fn release_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("releases"))
    }

    pub fn alias_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("aliases"))
    }

    pub fn download_dir(&self) -> PathBuf {
        self.ensure_create(self.snm_home().join("downloads"))
    }

    pub fn alias_default(&self) -> PathBuf {
        self.alias_dir().join("default")
    }

    pub fn bin_path(&self, path: PathBuf) -> PathBuf {
        if cfg!(unix) {
            path.join("bin")
        } else {
            path
        }
    }
}