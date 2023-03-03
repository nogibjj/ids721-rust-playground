extern crate reqwest;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Joke {
    value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.chucknorris.io/jokes/random";

    let joke: Joke = reqwest::get(url)
        .await?
        .json()
        .await?;

    println!("{}", joke.value);

    Ok(())
}
