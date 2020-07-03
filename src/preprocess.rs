use std::string::String;
use std::fs::File;
use tempfile::tempfile;
use std::io::{self, Write, BufRead, Seek, SeekFrom};

pub fn preprocess(in_file: &mut File) -> io::Result<File> {
    let mut tmp = tempfile()?;

    let in_iter = io::BufReader::new(in_file)
                      .lines()
                      .map(|x| x.unwrap())
                      .map(remove_comments)
                      .map(expand_pseudos)
                      .map(load_symbols)
                      .map(collect_whites);

    let mut line_num = 0;

    for line in in_iter {
        let line = line.as_str().trim();
        if line != "" {
            writeln!(tmp, "{}|{}", line_num, line)?;
            line_num += 1;
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
fn collect_whites(x: String) -> String {
    x.split_whitespace().collect::<Vec<&str>>().join(" ")
}
