use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptionError {
    #[error("Failed to read conf.toml")]
    ConfigFileReadError(#[from] std::io::Error),
    #[error("Failed to parse conf.toml")]
    ConfigFileConvertError(#[from] toml::de::Error),
    #[error("Failed to parse arg {}",.0)]
    WrongArgs(String),
    #[error("Config not complete")]
    ConfigNotComplete,
}
