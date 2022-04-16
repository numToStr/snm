use crate::cli::{Cli, Config};
use clap::{crate_name, IntoApp, Parser};
use clap_complete::{generate, shells};
use snm_core::{shell::ShellKind, SnmRes};

#[derive(Debug, Parser)]
pub struct Completions {
    /// Type of shell you want to configure
    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Completions {
    fn init(self, _: Config) -> SnmRes<()> {
        let name = crate_name!();
        let mut app = Cli::command();
        let mut stdout = std::io::stdout();

        match &self.shell {
            ShellKind::Bash => {
                generate(shells::Bash, &mut app, name, &mut stdout);
            }
            ShellKind::Zsh => {
                generate(shells::Zsh, &mut app, name, &mut stdout);
            }
            ShellKind::Fish => {
                generate(shells::Fish, &mut app, name, &mut stdout);
            }
            ShellKind::Pwsh => {
                generate(shells::PowerShell, &mut app, name, &mut stdout);
            }
        };

        Ok(())
    }
}
