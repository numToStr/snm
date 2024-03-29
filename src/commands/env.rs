use crate::cli::Config;
use clap::Parser;
use snm_core::{
    shell::{bash, fish, pwsh, zsh, Shell, ShellKind},
    SnmRes,
};

#[derive(Debug, Parser)]
pub struct Env {
    /// Add the shell script to run `snm use` on directory change
    #[clap(short, long)]
    use_on_cd: bool,

    /// Appends the snm path to the end of $PATH
    #[clap(short, long)]
    append: bool,

    /// Type of shell you want to configure
    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Env {
    fn init(self, config: Config) -> SnmRes<()> {
        let shell: &dyn Shell = match &self.shell {
            ShellKind::Bash => &bash::Bash,
            ShellKind::Zsh => &zsh::Zsh,
            ShellKind::Fish => &fish::Fish,
            ShellKind::Pwsh => &pwsh::Pwsh,
        };

        let path = config.bin_path(config.alias_default().as_ref());

        println!("{}", shell.path(&path, self.append));

        // println!("{}", shell.env_var("SNM_LOGLEVEL", &config.log_level));

        if self.use_on_cd {
            println!("{}", shell.use_on_cd());
        }

        Ok(())
    }
}
