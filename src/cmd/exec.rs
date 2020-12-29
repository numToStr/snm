use crate::config::Config;
use crate::pretty_error_msg;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;
use std::env;
use std::process::{self, Command, Stdio};

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Exec {
    version: Version,
    binary: String,
    args: Option<String>,
}

impl super::Command for Exec {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let path = {
            let dir = config.release_dir();
            let versions = NodeVersion::list_versions(&dir)?;
            let version = self.version.to_node_version(&versions)?;
            let bin_path = super::bin_path(dir.join(version.version_str()));
            let path_env = env::var("PATH")?;
            let mut splits: Vec<_> = env::split_paths(&path_env).collect();
            splits.insert(0, bin_path);
            env::join_paths(splits)?
        };

        let exit_status = Command::new(&self.binary)
            .args(&self.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .env("PATH", &path)
            .spawn()
            .map_err(|_| pretty_error_msg!("Can't spawn program {}", &self.binary.bold()))?
            .wait()
            .map_err(|_| anyhow::Error::msg("Failed to grab exit code"))?;

        process::exit(exit_status.code().unwrap_or(1));
    }
}
