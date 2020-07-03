mod preprocess;

use std::fs;
use std::string::String;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::error::Error;

pub struct Config {
    pub in_file: String,
    pub out_file: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let in_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file"),
        };

        let out_file = match args.next() {
            Some(arg) => arg,
            None => String::from("a.bin"),
        };

        Ok(Config { in_file, out_file })
    }
}


struct Label {
    name: String,
    addr: u16,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut in_file = fs::File::open(config.in_file)?;
    let tmp = preprocess::preprocess(&mut in_file)?;

    println!("Finished preprocessing");

    let mut out_file = fs::File::create(config.out_file)?;

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
