use std::collections::{VecDeque, HashSet};
use std::string::String;
use std::fs::File;
use regex::Regex;
use tempfile::tempfile;
use std::io::{self, Write, BufRead, Seek, SeekFrom};

struct RegexReplacer {
    regex: Regex,
    replace: String,
}

fn expand_includes(in_file_path: String) -> io::Result<File> {
    let mut incl_queue = VecDeque::<String>::new();
    let mut incl_set = HashSet::<String>::new();

    incl_queue.push_back(in_file_path.clone());
    incl_set.insert(in_file_path);

    let mut out_file = tempfile()?;

    while let Some(path) = incl_queue.pop_front() {
        let incl_file = File::open(path)?;
        for line in io::BufReader::new(incl_file).lines() {
            let line = line.unwrap();
            if line.starts_with(".include") {
                let new_path = line.split('"').nth(1).unwrap().to_string();
                if !incl_set.contains(&new_path) {
                    incl_queue.push_back(new_path.clone());
                    incl_set.insert(new_path);
                }
            } else {
                writeln!(out_file, "{}", line)?;
            }
        }
    }   
    out_file.seek(SeekFrom::Start(0))?;
    Ok(out_file)
}
    

fn expand_macros(in_file: File) -> io::Result<File> {
    let mut in_reader = io::BufReader::new(in_file);
    let mut str_buf = String::new();

    let mut regex_list = Vec::<RegexReplacer>::new();
    let line_splitter = Regex::new(r#"(?P<mac>[^"]+)"\s*"(?P<exp>[^"]+)"#).unwrap();
    let regex_creator = Regex::new(r#"[ ,]\w+"#).unwrap();

    while let Ok(nbytes) = in_reader.read_line(&mut str_buf) {
        if str_buf.starts_with(".define") {
            println!("found {}", str_buf);
            println
            //writeln!(out_file, "{}", str_buf.split(r"\n").collect::<Vec<&str>>().join("\n"));
        } else if nbytes == 0 {
            break;
        }
        str_buf.clear();
    }
    in_reader.seek(SeekFrom::Start(0))?;
    let mut out_file = tempfile()?;

    for line in in_reader.lines()
                         .map(|x| x.unwrap())
                         .filter(|x| !x.starts_with(".define")) {
        writeln!(out_file, "{}", line)?;
    }

    out_file.seek(SeekFrom::Start(0))?;
    Ok(out_file)
}


pub fn preprocess(in_file_path: String) -> io::Result<File> {
    let tmp = expand_macros(expand_includes(in_file_path)?)?;

    let in_iter = io::BufReader::new(tmp)
                      .lines()
                      .map(|x| x.unwrap())
                      .map(remove_comments)
                      .map(collect_whites);

    let mut out_file = tempfile()?;
    let mut line_num = 0;

    for line in in_iter {
        let line = line.as_str().trim();
        if line != "" {
            writeln!(out_file, "{}|{}", line_num, line)?;
            line_num += 1;
        }
    }
    out_file.seek(SeekFrom::Start(0))?;
    Ok(out_file)
}
    
fn remove_comments(x: String) -> String {
    x.split(';').next().unwrap().to_string()
}
fn collect_whites(x: String) -> String {
    x.split_whitespace().collect::<Vec<&str>>().join(" ")
}
