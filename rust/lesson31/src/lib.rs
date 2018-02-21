use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
   // pub fn new(args: &[String]) -> Result<Config, &'static str>  { 
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>  { 

        args.next();//discart the first one (program path)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

/*
        //Original implementation
        if args.len() < 3 {
            return Err("not enough arguments");
        }    
        let query = args[1].clone();
        let filename = args[2].clone();
*/    
        //read environment variable:
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("case {}", case_sensitive);
        Ok(Config { query, filename, case_sensitive })
    }
}


//fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let filename = args[2].clone();
//
//    Config { query, filename } 
//
//    /*
//        Whenever possible try to avoid using clone to 
//        fix ownership problems because of its runtime cost.
//        Clone will make a full copy of the data.
//    */
//}

pub fn run (config: Config) -> Result<(), Box<Error>> {
    //Box error is a trait object, it means teh function
    //will return a type that implements error
    //but we don't need to specify the type itself.
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?; //Returns the error

    println!("File text: \n\n{}\n", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
    //returning an Ok with () is the ideomatic way to indicate
    //we are calling run for its side-effect only and we don't 
    //actually need the return value of Ok.

}


fn search<'a> (query: &str, contents: &'a str ) -> Vec<&'a str> {

    //We can use Iterator Adaptors to improve this method
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()

/*
    //Original implementation
    let mut results = Vec::new();

    for line in contents.lines() {
        //the lines method is an interator
        if line.contains(query) {
            results.push(line);
        }
    }

    results
*/
}



fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();//now query is a String instead of &str
    let mut results = Vec::new();


    for line in contents.lines() {
        //the lines method is an interator
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
/*
    In Rust iterators are a zero cost abstraction 
    by which there is no additional runtime overhead 
    in using the implementation.
*/