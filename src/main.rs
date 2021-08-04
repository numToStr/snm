mod cli;
mod commands;
mod config;
mod lib;

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
