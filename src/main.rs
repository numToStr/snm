mod cli;
mod cmd;
mod config;
mod lib;
mod shell;
mod sysinfo;

use console::style;

fn main() {
    let app = cli::parse();

    let code = match app.cmd.exec(app.options) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{} :: {}", style("ERROR").red(), e.to_string());
            1
        }
    };

    std::process::exit(code)
}
