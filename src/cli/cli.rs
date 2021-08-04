use crate::cli::Config;
use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};

use super::SubCommand;

const VERSION_HELP: &str = r#"
Versions:

    Numeric version numbers can be complete or partial semver, with an optional leading 'v'.
    Versions can also be specified by their codename, prefixed with lts- or lts/.

    4.9.1, 8, v6.1                  Numeric versions
    >14, <10, ~15.0.0               Semver ranges
    lts/boron, lts/carbon           Codenames for release streams

Dotfiles:

    Version can also be defined in these files and can used with 'use' command in order.
    Only the version pattern which are defined above are supported.

    - .nvmrc
    - .node-version
    - package.json ('engines.node' field)
"#;

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    after_help = VERSION_HELP
)]
pub struct Cli {
    #[clap(flatten)]
    pub options: Config,

    #[clap(subcommand)]
    pub cmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::parse()
}
