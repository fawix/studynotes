extern crate curl_easybuilder;

/*extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

*/

use curl_easybuilder::EasyBuilder;
use std::error::Error;
use std::fmt;


//#[derive(Deserialize)]
struct Story {
    by: String,
    title: String,
}

impl fmt::Display for Story {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, by {}", self.title, self.by)
    }
}


fn callback () {

}

fn run () -> Result<(), Box<Error>> {

    /*let url = "https://hacker-news.firebaseio.com/v0/topstories.json";

    let handle = http::handle();

    let resp = handle.get(url).exec()?;
    let body = std::str::from_utf8(resp.get_body())?;

    println!("{}", body);*/

    let mut ids = Vec::new();

    let mut request = EasyBuilder::new();
    //let request = easy.url("https://hacker-news.firebaseio.com/v0/topstories.json")

/*
    let mut request = Easy::new();
    request.url("https://hacker-news.firebaseio.com/v0/topstories.json")?;

    let mut transfer = request.transfer();
    transfer.write_function( |data| {
        ids.extend_from_slice(data);
        Ok(data.len())
    })?;

    transfer.perform()?;


    for id in ids  {
        println!("{}", id);
    }
*/
    /*let mut ids = Vec::new();

    let mut transfer = request.transfer();
    transfer.write_function( |data| {
        ids.extend_from_slice(data);
        Ok(data.len())
    })?;
    let l = transfer.perform()?;

    println!("ids: {:?}", l);

*/

    Ok(())
}

fn main() {
    if let Err(e) = run () {
        eprintln!("Error: {}", e);
    }
}