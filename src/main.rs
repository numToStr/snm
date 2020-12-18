mod cli;
mod cmd;
mod downloader;
mod fetcher;
mod url;

use cli::Cli;

fn main() {
    let app = Cli::new();

    println!("CMD : {}", app.cmd.exec())
}
