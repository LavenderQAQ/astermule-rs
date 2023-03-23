use std::fs;

use clap::Parser;
use log::info;
use serde_derive::Deserialize;

use crate::error::OptionError;

const TOML_FILE: &'static str = "./src/option/conf.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub args: Args,
}

#[derive(Parser, Debug, Deserialize)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    #[arg(short, long)]
    pub address: Option<String>,

    #[arg(short, long)]
    pub port: Option<u32>,

    #[arg(short, long)]
    pub target: Option<String>,

    #[arg(short, long)]
    pub dag: Option<String>,
}

impl Config {
    // pub fn set_flag(&mut self) {
    //     self.address = Some(self.address.as_deref().unwrap_or("0.0.0.0").to_string());
    //     self.port = self.port.or(Some(8080));
    //     self.target = Some(self.target.as_deref().unwrap_or("/").to_string());
    //     self.dag = Some(self.dag.as_deref().unwrap_or("{}").to_string());
    // }

    pub fn new() -> Result<Self, OptionError> {
        let contents =
            fs::read_to_string(TOML_FILE).or_else(|e| Err(OptionError::ConfigFileReadError(e)))?;
        let config =
            toml::from_str(&contents).or_else(|e| Err(OptionError::ConfigFileConvertError(e)))?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_conf_toml() {
        let args = Config::new();

        println!("{:?}", args);
    }
}
