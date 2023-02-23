mod utils;

use clap::Parser;
use utils::cli::{Cli, Commands};
use utils::article;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Read(read_options) => {
            article::read(read_options).await?;
        }
        Commands::Search(search_options) => {
            article::search(search_options);
        }
    }

    Ok(())
}