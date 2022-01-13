use std::error::Error;
use std::fs;

// Configuration variables should be grouped into a single structure so that
// their purpose becomes more clear.
pub struct Config {
    pub query: String,
    pub fname: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let num_args = args.len();
        if num_args < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            fname: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in {}", config.query, config.fname);

    let contents = fs::read_to_string(config.fname)?;
    println!("Contents:\n{}", contents);

    // Returning () is the idiomatic way to indicate that we are calling a
    // function for its side effects only (doesn't return a value we need)
    Ok(())
}
