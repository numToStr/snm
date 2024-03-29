use crate::cli::Config;
use clap::Parser;
use console::style;
use snm_core::{
    version::{DistVersion, UserVersion},
    SnmRes,
};
use std::env;
use std::process::{self, Command, Stdio};

#[derive(Debug, Parser)]
pub struct Exec {
    /// Nodejs version needed for executing the following command
    version: UserVersion,

    /// Command that needs to be executed
    binary: String,

    /// Arguments for the command
    args: Vec<String>,
}

impl super::Command for Exec {
    fn init(self, config: Config) -> SnmRes<()> {
        let path = {
            let release_dir = config.release_dir();
            let version = DistVersion::match_version(&release_dir, &self.version)?;
            let bin_path = config.bin_path(release_dir.join(version.to_string()).as_ref());
            let path_env = env::var_os("PATH").ok_or_else(|| {
                anyhow::anyhow!(
                    "Unable to read environment variable {}",
                    style("$PATH").bold()
                )
            })?;

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
            .map_err(|_| anyhow::anyhow!("Can't spawn program {}", style(self.binary).bold()))?
            .wait()
            .map_err(|_| anyhow::anyhow!("Failed to grab exit code"))?;

        process::exit(exit_status.code().unwrap_or(1));
    }
}
