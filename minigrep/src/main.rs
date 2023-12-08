use std::{env, process};
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else( | err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

   if let Err(e) = minigrep::run(config) {
       println!("App error: {}",e);
       process::exit(1);
   };
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
   let mut results = Vec::new();

   for line in content.lines() {
       if line.contains(query) {
           results.push(line)
       }
   }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust\
safe, fast, productive.\
Pick three.\
";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }
}