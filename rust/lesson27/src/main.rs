extern crate lesson27;

use std::env;
use std::process;

use lesson27::Config;

fn main() {
    println!("Hello, modular rgreps!");

    let args: Vec<String> = env::args().collect();

    //what not to do:
    //let (query, filename) = parse_config(&args);
    //this is a primitive anti-pattern called primitive obsession.
    //hint: we group to return and break as soon as we receive something

    //we could make a function...
    //let config = lesson27::parse_config(&args);

    //but we can make a Constructor!
    let config = Config::new(&args).
            unwrap_or_else(|err| {//closure 
                    eprintln!("Problem parsing arguments: {}", err);
                    process::exit(1);
            });//or anonymous functions :) 
    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = lesson27::run(config) {
        //println!("Application error: {}", e);
        //println will always print to stdout 
        //to print to stderr : 
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
