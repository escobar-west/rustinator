mod preprocess;

use std::fs;
use std::string::String;
use std::io::{self, Write, BufRead};
use std::error::Error;

pub struct Config {
    pub in_file_path: String,
    pub out_file_path: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let in_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file"),
        };

        let out_file_path = match args.next() {
            Some(arg) => arg,
            None => String::from("a.bin"),
        };

        Ok(Config { in_file_path, out_file_path })
    }
}


struct Label {
    name: String,
    addr: u16,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tmp = preprocess::preprocess(config.in_file_path)?;

    println!("Finished preprocessing");

    let mut out_file = fs::File::create(&config.out_file_path)?;

    for line in io::BufReader::new(tmp).lines() {
        match line {
            Ok(data) => {
                println!("{}", data);
                writeln!(out_file, "{}", data)?;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}
