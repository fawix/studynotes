extern crate lesson27;

use std::env;
use std::process;

use lesson27::Config;

fn main() {
    println!("Hello, Improved modular rgreps!");

    /*
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).
            unwrap_or_else(|err| {//closure 
                    eprintln!("Problem parsing arguments: {}", err);
                    process::exit(1);
            });//or anonymous functions :) 
    */

    //We can improve the above using interator insted
    let config = Config::new(env::args()). //this return iterator to config new
            unwrap_or_else(|err| {//closure 
                    eprintln!("Problem parsing arguments: {}", err);
                    process::exit(1);
            });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = lesson27::run(config) {
        //println!("Application error: {}", e);
        //println will always print to stdout 
        //to print to stderr : 
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
