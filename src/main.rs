use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|e| {
    println!("Error!");
    process::exit(1);
  });

  if let Err(e) = run(config) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//   let file_contents = fs::read_to_string(config.filename)?;

//   Ok(())
// }

// struct Config {
//   query: String,
//   filename: String,
// }

// impl Config {
//   fn new(args: &[String]) -> Result<Config, &'static str> {
//     if args.len() < 3 {
//       return Err("Not enough arguments provided.");
//     }
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Ok(Config { query, filename })
//   }
// }
