use std::env;
use std::error::Error;
use std::fs;

// Configuration variables should be grouped into a single structure so that
// their purpose becomes more clear.
pub struct Config {
    pub query: String,
    pub fname: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let num_args = args.len();
        if num_args < 3 {
            return Err("not enough arguments");
        }
        // is_err() == true --> env var not set --> do case sensitive search
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: args[1].clone(),
            fname: args[2].clone(),
            case_sensitive: is_case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.fname)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    // Returning () is the idiomatic way to indicate that we are calling a
    // function for its side effects only (doesn't return a value we need)
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // creates new data (no longer a reference)
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn new_config_errs_with_2_args() {
        let args = [String::from("bin_name"), String::from("arg1")];

        let _config = Config::new(&args).unwrap_or_else(|err| {
            panic!("Argument parsing problem: {}", err);
        });
    }

    #[test]
    fn new_config_works_with_3_args() {
        let args = [
            String::from("bin_name"),
            String::from("arg1"),
            String::from("arg2"),
        ];

        if let Err(msg) = Config::new(&args) {
            panic!("Fatal test error: {}", msg);
        }
    }

    #[test]
    fn search_returns_1_result() {
        let query = "fear";
        let contents = "\
All my past and futures
And we all went to heaven in a little row boat
There was nothing to fear and nothing to doubt";
        assert_eq!(
            vec!["There was nothing to fear and nothing to doubt"],
            search(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive_2_results() {
        let query = "and";
        let contents = "\
Black-eyed angels swam with me
A moon full of stars and astral cars
And all the things I used to see";
        assert_eq!(
            vec![
                "A moon full of stars and astral cars",
                "And all the things I used to see"
            ],
            search_case_insensitive(query, contents)
        );
    }
}
