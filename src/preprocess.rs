use std::collections::{VecDeque, HashSet};
use std::string::String;
use std::fs::File;
use tempfile::tempfile;
use std::io::{self, Write, BufRead, Seek, SeekFrom};

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
                let new_path = String::from(line.split('"').nth(1).unwrap());
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
    

pub fn preprocess(in_file_path: String) -> io::Result<File> {
    let tmp = expand_includes(in_file_path)?;

    let in_iter = io::BufReader::new(tmp)
                      .lines()
                      .map(|x| x.unwrap())
                      .map(remove_comments)
                      .map(expand_pseudos)
                      .map(load_symbols)
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
    String::from(x.split(';').next().unwrap())
}
fn expand_pseudos(x: String) -> String {
    x
}
fn load_symbols(x: String) -> String {
    x
}
fn collect_whites(x: String) -> String {
    x.split_whitespace().collect::<Vec<&str>>().join(" ")
}
