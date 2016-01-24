use std::env;

pub struct Arguments {
    pub filename: String,
    pub max_threads: usize
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            filename: env::args().nth(1).unwrap(),
            max_threads: env::args().nth(2).unwrap_or("10".to_string()).parse::<usize>().unwrap()
        }
    }
}
