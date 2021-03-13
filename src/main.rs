// our tool should be able to take a searchtring followed by an example-filename.txt

use std::env; // like an import statement
use std::fs; // bring in the fs to read files

fn main() {
    // we capture the arguments by using the std(short for standard) library particularly the env module
    // the env module has a function called args() and this function returns an 'iterator'
    // we attach the .collect fn method and grab what the args return
    // we store in the args variable which has a type of Vector of Strings
    let args: Vec<String> = env::args().collect();

    // then we index into the vector and grab those strings
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // get the contents of the file
    let contents = fs::read_to_string(config.filename) // read_to_string returns a Result type
        .expect("Something went wrong reading the file"); // and that's why we can add the .expect on it

    println!("With text:\n{}", contents);
}

// making a Struct here to convey better meaning to the way parse_config works
struct Config {
    query: String,
    filename: String,
}

// Moving logic for parsing the args here
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // this first one will be the pattern we search for
    let filename = args[2].clone(); // this one will be the source file we're searching into

    Config { query, filename }
}
