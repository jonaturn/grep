use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");

    //other alternatives
    //OK(())
    //if u add semicolon it is evaluated and discarded
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        // we need the config struct to take ownership of the variables
        // if we used a tuple the tuple will be dropped together with the local scope, but configs can last longer
        //so we need to own the variables
        if (args.len() < 3) {
            // error handling for unexpected inputs
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        //using clone to fix ownership problems is simple but costly ch13 will teach other ways probably closures

        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //lifetimes: our result will live as long as the contents arg being passed in
    //If no lifetimes specified, the compiler dont know which arguments lifetime we need
    //if the compiler assumes we are making string slices out of query rather than content
    //the safe checking will handle incorrectly
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
    //return result;
}

#[cfg(test)]
//module for testing
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
