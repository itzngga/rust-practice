use std::{fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
   pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!")
        }
        let query = args[2].clone();

        if args.len() < 4 {
            return Err("not enough arguments!")
        }

        let filename = args[3].clone();

        return Ok(Config {query, filename})
   }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("With text: {}\n", content);

    Ok(())
}