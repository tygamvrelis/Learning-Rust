// Configuration variables should be grouped into a single structure so that
// their purpose becomes more clear.
pub struct Config {
    pub query: String,
    pub fname: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            fname: args[2].clone(),
        }
    }
}
