use crate::config::Config;
use crate::lib::{
    linker::Linker,
    version::{dist_version::DistVersion, user_version::UserVersion},
    SnmRes,
};
use clap::Clap;
use console::style;

#[derive(Debug, Clap)]
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

        let release_dir = config.release_dir();

        match version {
            UserVersion::Lts(alias) | UserVersion::Alias(alias) => {
                let alias_dir = config.alias_dir().join(&alias);

                if !alias_dir.exists() {
                    anyhow::bail!("Alias {} not found", style(alias).bold());
                }

                let dist_ver = Linker::read_convert_to_dist(&alias_dir, &release_dir)?;

                let dist_path = release_dir.join(dist_ver.to_string());

                Linker::create_link(&dist_path, &config.alias_default())?;

                println!("Using Alias {}", style(alias).bold());
            }
            version => {
                let dist_ver = DistVersion::match_version(&release_dir, &version)?;

                let dist_path = release_dir.join(dist_ver.to_string());

                Linker::create_link(&dist_path, &config.alias_default())?;

                println!("Using version {}", style(dist_ver).bold());
            }
        }

        Ok(())
    }
}
