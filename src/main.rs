mod alias;
mod archive;
mod cli;
mod cmd;
mod config;
mod downloader;
mod errors;
mod fetcher;
mod shell;
mod symlink;
mod sysinfo;
mod url;
mod version;

use colored::*;

fn main() {
    let app = cli::parse();

    if let Err(e) = app.cmd.exec(app.options) {
        eprintln!(
            "{} :: {}",
            "ERROR".bright_blue(),
            e.to_string().bright_red()
        )
    }
}
