use crate::cli::Config;
use clap::Parser;
use console::style;
use snm_core::{
    linker::Linker,
    version::{DistVersion, UserVersion},
    SnmRes,
};

#[derive(Debug, Parser)]
pub struct Use {
    /// Can be a partial semver or a LTS version name by the format lts/NAME.
    version: Option<UserVersion>,
}

impl super::Command for Use {
    fn init(self, config: Config) -> SnmRes<()> {
        let version = match self.version {
            Some(v) => v,
            None => UserVersion::from_file()?,
        };

        let r_dir = config.release_dir();

        match version {
            UserVersion::Lts(lts_code) => {
                let codename_dir = config.alias_dir().join(&lts_code.to_string());

                if !codename_dir.as_ref().exists() {
                    anyhow::bail!("Codename {} not found", style(lts_code).bold());
                }

                let dist_ver = Linker::read_convert_to_dist(&codename_dir, &r_dir)?;

                let dist_path = r_dir.join(dist_ver.to_string());

                Linker::create_link(&dist_path, &config.alias_default())?;

                println!("Using codename: {}", style(lts_code).bold());
            }
            UserVersion::Alias(alias) => {
                let alias_dir = config.alias_dir().join(alias.as_ref());

                if !alias_dir.as_ref().exists() {
                    anyhow::bail!("Alias {} not found", style(alias).bold());
                }

                let dist_ver = Linker::read_convert_to_dist(&alias_dir, &r_dir)?;

                let dist_path = r_dir.join(dist_ver.to_string());

                Linker::create_link(&dist_path, &config.alias_default())?;

                println!("Using alias: {}", style(alias).bold());
            }
            version => {
                let dist_ver = DistVersion::match_version(&r_dir, &version)?;

                let dist_path = r_dir.join(dist_ver.to_string());

                Linker::create_link(&dist_path, &config.alias_default())?;

                println!("Using version: {}", style(dist_ver).bold());
            }
        }

        Ok(())
    }
}
