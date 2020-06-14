use std::fs;
use std::fs::File;
use tempfile::tempfile;
use std::io::{self, Read, Write, BufRead, Seek, SeekFrom};
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
            None => return Err("Didn't get an output file"),
        };

        Ok(Config { in_file, out_file })
    }
}

pub fn preprocess(in_file: &mut File) -> io::Result<File> {
    let mut contents = String::new();
    in_file.read_to_string(&mut contents)?;

    let mut tmp = tempfile()?;

    tmp.write_all(contents.as_bytes())?;
    writeln!(tmp, "Test")?;
    writeln!(tmp, "Test")?;

    tmp.seek(SeekFrom::Start(0))?;
    Ok(tmp)
}
    

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut in_file = fs::File::open(config.in_file)?;
    let tmp = preprocess(&mut in_file)?;

    for line in io::BufReader::new(tmp).lines() {
        match line {
            Ok(data) => println!("{}", data),
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}
