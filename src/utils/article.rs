use clap::{arg, Args};
use reqwest::Client;
use serde::Deserialize;
use termimad::MadSkin;

use super::doc_view;

#[derive(Deserialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub content: String,
    pub timestamp: String
}

#[derive(Args)]
pub struct ReadOptions {
    /// Title of the article
    title: String,
}

#[derive(Args)]
pub struct SearchOptions {
    /// Search Query
    query: String,
    /// Number of results to return
    #[arg(short, long, default_value_t = 10)]
    items: u32
}

pub async fn read(read_options: &ReadOptions) -> Result<(), reqwest::Error> {
    let url: String = format!("https://decode.sh/raw/{}", read_options.title);
    let client: Client = reqwest::Client::new();

    let article: Article = client.get(url).send().await?.json().await?;

    let skin: MadSkin = doc_view::make_skin();
    let md: String = format!("# {}   Published: *{}*\n{}\n{}", article.title, article.timestamp, article.description, article.content);
    doc_view::run_app(skin, &md);

    Ok(())
}

pub fn search(search_options: &SearchOptions) {
    println!("Query: {}", search_options.query);
    println!("Items: {}", search_options.items);
}