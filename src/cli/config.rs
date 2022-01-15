use clap::Parser;
use snm_core::{
    types::{AliasDir, DownloadDir, ReleaseDir, SnmDir, UserAlias},
    MIRROR,
};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Parser, Debug)]
pub struct Config {
    /// Root directory of the snm installation
    #[clap(
        long,
        name = "base-dir",
        env = "SNM_DIR",
        global = true,
        default_value_t
    )]
    pub snm_dir: SnmDir,

    /// Nodejs release mirror
    #[clap(
        long = "node-dist-mirror",
        name = "mirror",
        env = "SNM_NODE_DIST_MIRROR",
        default_value = MIRROR,
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

    pub fn release_dir(&self) -> ReleaseDir {
        let p = self.ensure_create(self.snm_dir.as_ref().join("releases"));

        ReleaseDir::new(p)
    }

    pub fn alias_dir(&self) -> AliasDir {
        let p = self.ensure_create(self.snm_dir.as_ref().join("aliases"));

        AliasDir::new(p)
    }

    pub fn download_dir(&self) -> DownloadDir {
        let p = self.ensure_create(self.snm_dir.as_ref().join("downloads"));

        DownloadDir::new(p)
    }

    pub fn alias_default(&self) -> AliasDir {
        self.alias_dir().join(UserAlias::ACTIVE)
    }

    pub fn bin_path(&self, path: &Path) -> PathBuf {
        if cfg!(unix) {
            path.join("bin")
        } else {
            path.to_path_buf()
        }
    }
}
