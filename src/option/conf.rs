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

impl crate::option::Option for Config {
    fn parse() -> Result<crate::option::Args, OptionError> {
        let contents =
            fs::read_to_string(TOML_FILE).or_else(|e| Err(OptionError::ConfigFileReadError(e)))?;
        let mut config: Config =
            toml::from_str(&contents).or_else(|e| Err(OptionError::ConfigFileConvertError(e)))?;

        let cmd_args = Args::parse();

        match config.args {
            Args {
                address: Some(_),
                port: Some(_),
                target: Some(_),
                dag: Some(_),
            } => {}
            _ => {
                return Err(OptionError::ConfigNotComplete);
            }
        }

        if let Some(address) = cmd_args.address {
            config.args.address = Some(address);
        }

        if let Some(port) = cmd_args.port {
            config.args.port = Some(port);
        }

        if let Some(target) = cmd_args.target {
            config.args.target = Some(target);
        }

        if let Some(dag) = cmd_args.dag {
            config.args.dag = Some(dag);
        }

        Ok(crate::option::Args {
            address: config
                .args
                .address
                .ok_or(OptionError::WrongArgs("address".to_string()))?,
            port: config
                .args
                .port
                .ok_or(OptionError::WrongArgs("port".to_string()))?,
            target: config
                .args
                .target
                .ok_or(OptionError::WrongArgs("target".to_string()))?,
            dag: config
                .args
                .dag
                .ok_or(OptionError::WrongArgs("dag".to_string()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::option::Option;

    use super::*;

    #[test]
    fn test_config_parse() {
        let args = Config::parse();
        assert!(args.is_ok())
    }
}
