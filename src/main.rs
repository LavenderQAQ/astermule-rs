use anyhow::Result;
use astermule_rs::{option, option::Option};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let config = option::Config::parse()?;
    println!("{:?}", config);

    Ok(())
}
