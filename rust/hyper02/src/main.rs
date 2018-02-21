extern crate reqwest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io::Read;
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

//Terribly slow... 
fn run () -> Result<(), Box<Error>> {

    let url = String::from("https://hacker-news.firebaseio.com/v0/topstories.json");
    let mut response = reqwest::get(&url)?;

    let ids: Vec<i32> = serde_json::from_str(&response.text()?)?;

    for id in ids {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json",id);
        let mut response = reqwest::get(&url)?;
        let story: Story = serde_json::from_str(&response.text()?)?;

        println!("{}", story);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run () {
        eprintln!("{}", e);
    }
}
