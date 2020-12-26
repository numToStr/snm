mod alias;
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
mod xtract;

fn main() -> anyhow::Result<()> {
    let app = cli::parse();

    app.cmd.exec(app.options)
}
