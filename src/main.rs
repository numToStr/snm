mod cli;
mod cmd;
mod config;
mod directory;
mod downloader;
mod fetcher;
mod shell;
mod url;
mod xtract;

use cli::Cli;

fn main() {
    let app = Cli::new();

    println!("CMD : {}", app.cmd.exec(app.options))
}
