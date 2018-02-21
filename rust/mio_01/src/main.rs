extern crate mio;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::fmt;

use mio::net::{TcpListener, TcpStream};

#[derive(Deserialize)]
struct Story {
    by: String,
    id: i32,
    score: i32,
    time: i32,
    title: String,
}

impl fmt::Display for Story {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, by {}", self.title, self.by)
    }
}

fn run () -> Result<(), Box<Error>> {


    Ok(())
}

fn main() {
    if let Err(e) = run () {
        eprintln!("Error: {}", e);
    }
}