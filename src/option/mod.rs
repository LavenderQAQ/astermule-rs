use crate::error::OptionError;

mod conf;

pub struct Args {
    pub address: String,
    pub port: u32,
    pub target: String,
    pub dag: String,
}

pub trait Option {
    fn parse() -> Result<Args, OptionError>;
}
