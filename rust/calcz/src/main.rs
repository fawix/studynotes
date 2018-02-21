extern crate calcz;

use std::process;

fn main() {
    println!("Toy Calculator! Just for fun :)");

    if let Err(e) =  calcz::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
