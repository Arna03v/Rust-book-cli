// the first task is to make minigrep accept 2 command line arguments, a file path and a string to search for
// That is, we want to be able to run our program with cargo run

// we are creating a version of grep : which at its base level allows us us to search for a string within a file

use std::env;
use std::process;

// now to import config from lib crate
use minigrep::Config;
fn main(){
    
    let args : Vec<String> = env::args().collect(); // gives us an iterator over the arguments passed over our program, collect() turns that iterator into a collection, here a vec<String>
    
    // let query = &args[1]; // as we dont care about the first index which is the  binary path
    // let filename = &args[2];

    let config = Config::new(&args).unwrap_or_else(|err|{
        // in the "Ok" case, it will return the value stored in "Ok"
        // when an err occurs, this block (closure) is executed
        eprintln!("Problem parsing arguments: {}", err); // printing errors to the error stream and not the output file
        process::exit(1);
    }); 

    println!("Searching for '{}' in file {} \n", config.query, config.filename);
    

    // println!("{:?}", args); 
    // if we dont pass in any arguments, we get a path to our binary as default
    // whatever arguments we pass get added to the vector of strings aas strings.

    if let Err(e ) = minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    // refactoring our code by putting our functions in the library crate and calling fnctions from there

}

