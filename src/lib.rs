use std::error::Error;
use std::fs;

// making a Struct here to convey better meaning to the way parse_config works
pub struct Config {
    pub query: String,
    pub filename: String,
}
// adding and implementation so we can later create new instances of Config
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // if we don't get the args
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone(); // this first one will be the pattern we search for
        let filename = args[2].clone(); // this one will be the source file we're searching into

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // get the contents of the file
    let contents = fs::read_to_string(config.filename)?; // <- the ? here means that if it errors out, it will handle it.


    println!("With text:\n{}", contents);
    Ok(())
}