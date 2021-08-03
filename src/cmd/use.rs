use crate::config::Config;
use crate::lib::{
    linker::Linker,
    version::{dist_version::DistVersion, user_version::UserVersion, ParseVersion},
    SnmRes,
};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
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
                    anyhow::bail!("Alias {} not found", alias);
                }

                let link_path = Linker::read_link(&alias_dir)?;
                let link_ver = link_path.strip_prefix(&release_dir)?.to_str();

                if let Some(v) = link_ver {
                    let dist_ver = DistVersion::parse(v)?;

                    let dist_path = release_dir.join(dist_ver.to_string());

                    Linker::new(&dist_path).create_link(&config.alias_default())?;

                    println!("Using Alias {}", &alias);
                }
            }
            version => {
                let dist_ver = DistVersion::match_version(&release_dir, &version)?;

                let dist_path = release_dir.join(dist_ver.to_string());

                Linker::new(&dist_path).create_link(&config.alias_default())?;

                println!("Using Node.js {}", dist_ver);
            }
        }

        Ok(())
    }
}
