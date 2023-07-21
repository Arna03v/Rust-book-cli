// root of our library crate

use std::error::Error;
use std::fs; // for file systems
use std::env;

// fn main(){}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{// returning any type of error when an error occurs 
    // this function is for code which has nothing to do with setting up and handling errors

    // pasting the logic that reads the file from main
    let contents = fs::read_to_string(config.filename)?;
    // if an error is returned, it is return from the Error type 
    // read_to_string takes in a path to a file and returns a result type, if an error occurs, expec() gives custom error

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };

    let mut i = 1;

    println!("The following lines contain your query: \n");
    println!("CASE_SENSITIVE: {} \n", config.case_sensitive);

    for line in results{
        // printing the line that cnatins th query  

        println!("{i}. {line}");
        i += 1;
    }
    Ok(())
}

pub struct Config{
    // to show that query and filename are connected
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, // if we want the search function to be case sensitive or not
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str>{
        // if Ok then it returns a COnfig struct, or else an error message

        if args.len() < 3{
            return Err("Not enough arguments, first argument should be the query_name and the second argument should be the file_name");
        }
        // to extract the query and filename from the args parameter
        let query = args[1].clone(); 
        let filename = args[2].clone(); // as we want to pass strings but dont want to take ownwership of the strings    
        

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // if case insennsitive is not set which will make the variable false
        Ok(Config {query, filename, case_sensitive})
    
        // now sice this function is very closesly tied to the COnfig struct, we use it in the impl block
    }    
    
}

// we define the serach function to return an empty array for now
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insenstive(){
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three,
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,contents),
        )
    }
}  