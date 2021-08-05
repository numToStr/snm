use crate::cli::Config;
use clap::Clap;
use console::style;
use snm_core::{linker::Linker, version::DistVersion, SnmRes};

#[derive(Debug, Clap)]
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
                        println!("> {} \t{}", style(version).bold(), a.join(", "));
                    } else {
                        println!("- {} \t{}", version, a.join(", "));
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
