extern crate chttp;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use chttp::{Client, http, Options};

use std::error::Error;
use std::fmt;



#[derive(Deserialize)]
struct Story {
    by: String,
    title: String,
}

impl fmt::Display for Story {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, by {}", self.title, self.by)
    }
}

fn run () -> Result<(), Box<Error>> {

    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";

    let client = Client::builder()
                    .max_connections(Some(8))
                    .build();

    let mut resp = client.get(url)?;
    let ids: Vec<i32> = serde_json::from_str(&resp.body_mut().text()?)?;

    for id in ids {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        resp = client.get(&url)?;

        let story: Story = serde_json::from_str(&resp.body_mut().text()?)?;
        println!("{}", story);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run () {
        eprintln!("Error: {}", e);
    }
}