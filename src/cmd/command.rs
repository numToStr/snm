use crate::config::Config;
use anyhow::Result;

pub trait Command {
    type InitResult;

    fn init(&self, config: Config) -> Result<Self::InitResult>;
}
