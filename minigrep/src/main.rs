use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}\n", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    println!("Search {}.", config.query);
    println!("Target file is {}.\n", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Failed to read the file.");

    println!("Contents: \n{}", contents);
    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone(); // clone(): not efficient
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

