use crate::config::Config;
use crate::shell::{fish, zsh, Shell, ShellKind};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Env {
    #[clap(short, long)]
    use_on_cd: bool,

    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Env {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let shell: Box<&dyn Shell> = match &self.shell {
            ShellKind::Zsh => Box::new(&zsh::Zsh),
            ShellKind::Fish => Box::new(&fish::Fish),
        };

        println!("{}", shell.path_env(&config.alias_default().join("bin")));

        println!(
            "{}",
            shell.env_var("SNM_DIR", &config.snm_home().to_str().unwrap_or(""))
        );

        println!("{}", shell.env_var("SNM_LOGLEVEL", &config.log_level));

        Ok(())
    }
}
