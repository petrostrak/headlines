use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;
    dbg!(articles);
    todo!()
}

fn main() {
    let url =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=7df214ecd666464198e34e3ca13f77bd";

    let articles = get_articles(url);
}
