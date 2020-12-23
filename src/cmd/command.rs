use crate::config::Config;

pub trait Command {
    fn init(&self, config: Config) -> ();
}
