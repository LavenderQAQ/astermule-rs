use anyhow::Result;
use astermule_rs::{option, option::Option};

#[tokio::main]
async fn main() -> Result<()> {
    let config = option::Config::parse()?;
    println!("{:?}", config);

    Ok(())
}
