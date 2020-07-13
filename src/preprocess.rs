use std::collections::{VecDeque, HashSet};
use std::borrow::Cow;
use std::string::String;
use std::fs::File;
use regex::Regex;
use tempfile::tempfile;
use std::io::{self, Write, BufRead, Seek, SeekFrom};

struct RegexReplacer {
    regex: Regex,
    replace: String,
}

impl RegexReplacer {
    fn is_match(&self, line: &str) -> bool {
        if self.regex.is_match(&line) {
            println!("\nfound match {}", line);
        }
        self.regex.is_match(&line)
    }

    fn replace_macro(&self, line: &str) -> String {
        println!("replacing macro line {}", &line);
        let caps = self.regex.captures(line).unwrap();
        println!("captured {}", caps.get(0).map_or("NOTHING", |m| m.as_str()));
        let mut out_string = String::new();
        caps.expand(&self.replace, &mut out_string);
        println!("returning {}", &out_string);
        out_string
    }
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
    let line_regex = Regex::new(r#""(?P<mac>[^"]+)"\s*"(?P<exp>[^"]+)""#).unwrap();
    let word_regex = Regex::new(r"(?P<arg>\w+)").unwrap();

    while let Ok(nbytes) = in_reader.read_line(&mut str_buf) {
        if str_buf.starts_with(".define") {
            let captured_macro_line = line_regex.captures(&str_buf).unwrap();

            let mut macro_vec = captured_macro_line["mac"]
                                    .split_whitespace()
                                    .collect::<Vec<&str>>();

            let args_regex: String;
            let mut parsed_tokens = String::new();
            let mut exp_repl = String::new();

            if let Some(args) = macro_vec.get_mut(1) {
                parsed_tokens = args.split(',').collect::<Vec<&str>>().join("|");
                parsed_tokens = format!(r"(?P<pre>^|[ ,])(?P<mid>{})(?P<end>$|[ ,\\])", &parsed_tokens);

                args_regex = word_regex.replace_all(*args, r"(?P<${arg}>\w+)").to_string();
                *args = &args_regex;
            } else {
                parsed_tokens = format!(r"(?P<pre>^|[ ,])(?P<mid>{})(?P<end>$|[ ,\\])", macro_vec[0]);
                println!("parsed_tokens are {}", &parsed_tokens);
            }
                let exp_regex = Regex::new(&parsed_tokens).unwrap();
                exp_repl = exp_regex.replace_all(&captured_macro_line["exp"], "${pre}$${${mid}}${end}").to_string();
                println!("exp_repl is {}", &exp_repl);

                let macro_regex = Regex::new(&macro_vec.join(" ")).unwrap();
                println!("creating macro_regex {}", macro_regex);
                regex_list.push(RegexReplacer { regex: macro_regex, replace: exp_repl });
        
        } else if nbytes == 0 {
            break;
        }
        str_buf.clear();
    }
    in_reader.seek(SeekFrom::Start(0))?;
    let mut out_file = tempfile()?;

    for mut line in in_reader.lines().map(|x| x.unwrap()).filter(|x| !x.starts_with(".define")) {
        for reg in &regex_list {
            if reg.is_match(&line) {
                line = reg.replace_macro(&line);
                break;
            }
        }
            
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
