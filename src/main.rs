// our tool should be able to take a searchtring followed by an example-filename.txt

use std::env; // like an import statement
use std::process; // bringing in to to do exit codes
use minigrep::Config;

fn main() {
    // we capture the arguments by using the std(short for standard) library particularly the env module
    // the env module has a function called args() and this function returns an 'iterator'
    // we attach the .collect fn method and grab what the args return
    // we store in the args variable which has a type of Vector of Strings
    let args: Vec<String> = env::args().collect();

    // then we index into the vector and grab those strings
    let config = Config::new(&args).unwrap_or_else( |err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// Moving logic for parsing the args here
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone(); // this first one will be the pattern we search for
//     let filename = args[2].clone(); // this one will be the source file we're searching into
//
//     Config { query, filename }
// }
