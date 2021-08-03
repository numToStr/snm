mod cli;
mod cmd;
mod config;
mod lib;
mod shell;
mod sysinfo;

use colored::*;

fn main() {
    let app = cli::parse();

    let code = match app.cmd.exec(app.options) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{} :: {}", "ERROR".bright_red(), e.to_string());
            1
        }
    };

    std::process::exit(code)
}
