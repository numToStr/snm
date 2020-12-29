use crate::cli::Cli;
use crate::config::Config;
use crate::shell::ShellKind;
use clap::{crate_name, Clap, IntoApp};
use clap_generate::{
    generate,
    generators::{Bash, Fish, PowerShell, Zsh},
};
use std::io;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Completions {
    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Completions {
    type InitResult = ();

    fn init(&self, _: Config) -> anyhow::Result<Self::InitResult> {
        let name = crate_name!();

        match &self.shell {
            ShellKind::Bash => {
                generate::<Bash, _>(&mut Cli::into_app(), name, &mut io::stdout());
            }
            ShellKind::Zsh => {
                generate::<Zsh, _>(&mut Cli::into_app(), name, &mut io::stdout());
            }
            ShellKind::Fish => {
                generate::<Fish, _>(&mut Cli::into_app(), name, &mut io::stdout());
            }
            ShellKind::Pwsh => {
                generate::<PowerShell, _>(&mut Cli::into_app(), name, &mut io::stdout());
            }
        };

        Ok(())
    }
}
