extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use futures::{Future, Stream};
use hyper::{Client, Chunk};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use std::error::Error;
use std::io;
use std::fmt;

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

    let mut core = Core::new()?;

    let client = Client::configure()
                    .connector(HttpsConnector::new(8, &core.handle())?)
                    .build(&core.handle());

    let url = "https://hacker-news.firebaseio.com/v0/topstories.json".parse()?;
    let request = client.get(url).and_then(|response| {
        response.body().concat2().and_then(move |body: Chunk| {
            let v_stream = request.map() = serde_json::from_slice(&body).map_err(|e|{
                io::Error::new(io::ErrorKind::Other, e)
            })?;
            Ok(v)
        })
    });

    let ids: Vec<i32> = core.run(request)?;

    for id in ids {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id).parse()?;
        let request = client.get(url).and_then(|response| {
            response.body().concat2().and_then(move |body: Chunk| {
                let v: Story = serde_json::from_slice(&body).map_err(|e|{
                    io::Error::new(io::ErrorKind::Other, e)
                })?;
                //println!("{}",v);
                Ok(v)
            })
        });

        println!("{}", core.run(request)?);
    }


    Ok(())
}

/*
//Terribly slow... 
fn run2 () -> Result<(), Box<Error>> {

    let mut core = Core::new()?;

    let client = Client::configure()
                    .connector(HttpsConnector::new(8, &core.handle())?)
                    .build(&core.handle());

    let url = "https://hacker-news.firebaseio.com/v0/topstories.json".parse()?;
    let request = client.get(url).map(|v| stream::iter_ok(v)).flatten_stream();
    / *.and_then(|response| {
        response.body().concat2().and_then(move |body: Chunk| {
            let story_id_stream = request.map(|v| stream::iter_ok(v)) serde_json::from_slice(&body).map_err(|e|{
                io::Error::new(io::ErrorKind::Other, e)
            })?;
            Ok(v)
        })
    });

    let ids: Vec<i32> = core.run(request)?;

    for id in ids {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id).parse()?;
        let request = client.get(url).and_then(|response| {
            response.body().concat2().and_then(move |body: Chunk| {
                let v: Story = serde_json::from_slice(&body).map_err(|e|{
                    io::Error::new(io::ErrorKind::Other, e)
                })?;
                //println!("{}",v);
                Ok(v)
            })
        });
    * /
        println!("{}", core.run(request)?);
    }

    Ok(())
}*/

fn main() {
    if let Err(e) = run () {
        eprintln!("Error: {}", e);
    }
}