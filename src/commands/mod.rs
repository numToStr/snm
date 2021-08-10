pub mod alias;
pub mod completions;
pub mod env;
pub mod exec;
pub mod install;
pub mod latest;
pub mod ls;
pub mod ls_remote;
pub mod lts;
pub mod purge;
pub mod unalias;
pub mod uninstall;
pub mod r#use;
pub mod which;

use crate::cli::Config;
use snm_core::SnmRes;

pub trait Command {
    fn init(self, config: Config) -> SnmRes<()>;
}
