use crate::config::Config;
use crate::shell;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub enum ShellKind {
    /// Setup the zsh shell environment
    Zsh(shell::zsh::Zsh),

    /// Setup the fish shell environment
    Fish(shell::fish::Fish),
}

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct EnvConfig {
    #[clap(short, long)]
    use_on_cd: bool,
}

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Env {
    #[clap(flatten)]
    env_config: EnvConfig,

    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Env {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let shell: Box<&dyn shell::Shell> = match &self.shell {
            ShellKind::Zsh(m) => Box::new(m),
            ShellKind::Fish(m) => Box::new(m),
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
