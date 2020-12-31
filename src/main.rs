mod alias;
mod archive;
mod cli;
mod cmd;
mod config;
mod downloader;
mod fetcher;
mod shell;
mod symlink;
mod sysinfo;
mod url;
mod version;

use colored::*;

fn main() {
    let app = cli::parse();

    let code = match app.cmd.exec(app.options) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!(
                "{} :: {}",
                "ERROR".bright_blue(),
                e.to_string().bright_red()
            );
            1
        }
    };

    std::process::exit(code)
}
