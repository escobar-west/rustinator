use std::fs;
use std::string::String;
use std::fs::File;
use tempfile::tempfile;
use std::io::{self, Write, BufRead, Seek, SeekFrom};
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
    let mut tmp = tempfile()?;

    let in_iter = io::BufReader::new(in_file)
                      .lines()
                      .map(|x| x.unwrap())
                      .map(|x| remove_comments(x))
                      .map(|x| expand_pseudos(x))
                      .map(|x| load_symbols(x));

    let mut line_num = 0;

    for line in in_iter {
        let line = line.as_str().trim();
        match line {
            "" => (),
            _ =>  {
                writeln!(tmp, "{}|{}", line_num, line)?;
                line_num += 1;
            }
        }
    }
    tmp.seek(SeekFrom::Start(0))?;
    Ok(tmp)
}
    
fn remove_comments(x: String) -> String {
    String::from(x.split(';').next().unwrap())
}
fn expand_pseudos(x: String) -> String {
    x
}
fn load_symbols(x: String) -> String {
    x
}

struct Label {
    name: String,
    addr: u16,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut in_file = fs::File::open(config.in_file)?;
    let tmp = preprocess(&mut in_file)?;

    println!("Finished preprocessing");

    for line in io::BufReader::new(tmp).lines() {
        match line {
            Ok(data) => println!("{}", data),
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}
