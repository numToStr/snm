use crate::cli::{Cli, Config};
use clap::{crate_name, IntoApp, Parser};
use clap_generate::{generate, generators};
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
        let mut app = Cli::into_app();
        let mut stdout = std::io::stdout();

        match &self.shell {
            ShellKind::Bash => {
                generate(generators::Bash, &mut app, name, &mut stdout);
            }
            ShellKind::Zsh => {
                generate(generators::Zsh, &mut app, name, &mut stdout);
            }
            ShellKind::Fish => {
                generate(generators::Fish, &mut app, name, &mut stdout);
            }
            ShellKind::Pwsh => {
                generate(generators::PowerShell, &mut app, name, &mut stdout);
            }
        };

        Ok(())
    }
}
