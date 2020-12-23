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

fn main() -> anyhow::Result<()> {
    let app = Cli::new();

    app.cmd.exec(app.options)
}
