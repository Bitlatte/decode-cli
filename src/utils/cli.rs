use clap::{Parser, Subcommand};

use super::article;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Read a specific article
    Read(article::ReadOptions),
    /// Search for articles
    Search(article::SearchOptions)
}