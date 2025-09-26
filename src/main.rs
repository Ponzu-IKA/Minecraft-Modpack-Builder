use clap::Parser;
use config::Args;

mod build;
mod config;
mod utils;

#[cfg(test)]
mod test;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("Hello, world!");

    Ok(())
}
