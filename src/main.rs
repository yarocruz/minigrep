// our tool should be able to take a searchtring followed by an example-filename.txt

use std::env; // like an import statement

fn main() {
    // we capture the arguments by using the std(short for standard) library particularly the env module
    // the env module has a function called args() and this function returns an 'iterator'
    // we attach the .collect fn method and grab what the args return
    // we store in the args variable which has a type of Vector of Strings
    let args: Vec<String> = env::args().collect();

    // then we index into the vector and grab those strings
    let query = &args[1]; // this first one will be the pattern we search for
    let filename = &args[2]; // this one will be the source file we're searching into

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
