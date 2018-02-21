use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, rgrep!");
    //Following the book lesson chapter 12

    //Super odd the way Rust handle arguments...
    //It's not passed to main, instead 
    //it's stored in std::env::args collection...
    let args: Vec<String> = env::args().collect(); 
    //the collect is just to make a vector of all the values in args
    println!("{:?}", args);
    //The first argument is always the program path (or exectution path)

    //the below is sort of cheating lol :) 
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {} in {}", query, filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
     .expect("could not read the file");

    println!("File text: \n\n{}\n", contents);

    /*
        Rust Community Guidelines for separation of concerns:alloc

            1. Split your program into both a main.rs and a lib.rs and move your program’s logic into lib.rs.
            2. While your command line parsing logic is small, it can remain in main.rs.
            3. When the command line parsing logic starts getting complicated, extract it from main.rs into lib.rs as well.
            4. The responsibilities that remain in the main function after this process should be limited to:
                - Calling the command line parsing logic with the argument values
                - Setting up any other configuration
                - Calling a run function in lib.rs
                - If run returns an error, handling that error

        Check lesson27 for continuation...
*/

}