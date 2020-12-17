mod cli;
mod url;
mod versions;

use versions::Releases;

fn main() {
    let matches = cli::app();

    match matches.subcommand_name() {
        Some("lts") => {
            let mut body = Releases::fetch();

            println!("LTS :: {:#?}", body.lts().unwrap())
        }
        Some("latest") => {
            let mut body = Releases::fetch();

            println!("Latest :: {:#?}", body.latest().unwrap())
        }
        _ => {
            println!("Oops no command is found")
        }
    }
}
