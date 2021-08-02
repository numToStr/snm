use crate::cli::Cli;
use crate::config::Config;
use crate::shell::ShellKind;
use clap::{crate_name, Clap, IntoApp};
use clap_generate::{
    generate,
    generators::{Bash, Fish, PowerShell, Zsh},
};

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Completions {
    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Completions {
    type InitResult = ();

    fn init(&self, _: Config) -> anyhow::Result<Self::InitResult> {
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
