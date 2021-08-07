use crate::cli::Cli;
use crate::cli::Config;
use clap::{crate_name, Clap, IntoApp};
use clap_generate::{
    generate,
    generators::{Bash, Fish, PowerShell, Zsh},
};
use snm_core::{shell::ShellKind, SnmRes};

#[derive(Debug, Clap)]
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
                generate::<Bash, _>(&mut app, name, &mut stdout);
            }
            ShellKind::Zsh => {
                generate::<Zsh, _>(&mut app, name, &mut stdout);
            }
            ShellKind::Fish => {
                generate::<Fish, _>(&mut app, name, &mut stdout);
            }
            ShellKind::Pwsh => {
                generate::<PowerShell, _>(&mut app, name, &mut stdout);
            }
        };

        Ok(())
    }
}
