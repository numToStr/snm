mod cli;
mod url;
mod versions;

use cli::{Cli, SubCommand};
use versions::Releases;

fn main() {
    let matches = Cli::new();

    match matches.cmd {
        SubCommand::Lts => {
            let mut body = Releases::fetch();

            println!("LTS :: {:#?}", body.lts().unwrap())
        }
        SubCommand::Latest => {
            let mut body = Releases::fetch();

            println!("Latest :: {:#?}", body.latest().unwrap())
        }
        _ => {
            println!("Oops no command is found")
        }
    }
}
