use crate::config::Config;
use crate::lib::{linker::Linker, version::dist_version::DistVersion, SnmRes};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Ls;

impl super::Command for Ls {
    fn init(self, config: Config) -> SnmRes<()> {
        let release_dir = config.release_dir();

        let versions = DistVersion::list_versions(&release_dir)?;

        let aliases = Linker::list_aliases(&config.alias_dir(), &release_dir)?;

        for version in versions.into_iter() {
            match aliases.get(&version) {
                Some(a) => {
                    if a.contains(&"default".to_string()) {
                        println!("> {}\t{}", version, a.join(", "));
                    } else {
                        println!("- {}\t{}", version, a.join(", "));
                    }
                }
                _ => {
                    println!("- {}", version);
                }
            }
        }

        Ok(())
    }
}
