use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_contents = fs::read_to_string(config.filename)?;

  Ok(())
}

#[derive(Debug)]
pub struct Config {
  query: String,
  filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments provided.");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

#[cfg(test)]
mod test {
  use crate::*;

  #[test]
  fn test_config_constructor() {
    let args = [
      "program".to_string(),
      "arg1".to_string(),
      "arg2".to_string(),
    ];
    let config = Config::new(&args).unwrap();
    assert_eq!(config.query, "arg1".to_string());
    assert_eq!(config.filename, "arg2".to_string())
  }

  #[test]
  fn test_config_constructor_invalid() {
    let args = [String::from("a")];
    assert_eq!(
      Config::new(&args).unwrap_err(),
      "Not enough arguments provided."
    );
  }
}
