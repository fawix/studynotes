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
    pub fn new(args: &[String]) -> Result<Config, &'static str>  { 

        if args.len() < 3 {
            return Err("not enough arguments");
            //instead of panickign we can show a 
            //proper error message for the user
        }    
    
        let query = args[1].clone();
        let filename = args[2].clone();
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
    //we need a lifetime, because the return of vector references
    //a str line in the input of contents
    let mut results = Vec::new();

    for line in contents.lines() {
        //the lines method is an interator
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn more_than_one_result () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Build great Products.";

        assert_eq!(
            vec!["safe, fast, productive.", "Build great Products." ],
            search(query, contents)
        );
    }

    #[test]
    fn one_case_sensitive () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Build great Ducts.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_insensitive () {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}