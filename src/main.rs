mod cli;
mod cmd;

use cli::Cli;

fn main() {
    let app = Cli::new();

    println!("CMD : {}", app.cmd.exec())
}
