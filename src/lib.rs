use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    println!("{:#?}", contents);
    Ok(())
}

#[cfg(test)]
mod config_tests {
    use super::*;
    #[test]
    fn config_new_should_succeed() {
        let result_config = Config::new(&[String::from("lib.rs"), String::from("test"), String::from("test.txt")]).unwrap();
        let expected_config = Config {
            query: String::from("test"),
            filename: String::from("test.txt"),
        };
        assert_eq!(result_config.query, expected_config.query);
        assert_eq!(result_config.filename, expected_config.filename);
    }
    
    #[test]
    #[should_panic]
    fn config_new_should_fail() {
        Config::new(&[String::from("test"), String::from("test.txt")]).unwrap();
    }
}
