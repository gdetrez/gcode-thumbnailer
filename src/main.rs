use std::{
    fs,
    io::{prelude::*, BufReader},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    input: String,
    output: String,
}

fn main() {
    let opt = Opt::from_args();
    let input = opt.input.trim_start_matches("file://");
    let output = opt.output.trim_start_matches("file://");
    let file = fs::File::open(&input).unwrap();
    let bufreader = BufReader::new(file);
    let mut inthumbnail = false;
    let mut thumbnail: String = String::new();
    for line in bufreader.lines() {
        let line = line.unwrap();
        match line.strip_prefix("; ") {
            Some(l) if l.starts_with("thumbnail begin") => {
                inthumbnail = true;
                thumbnail = String::new()
            }
            Some(l) if l.starts_with("thumbnail end") => inthumbnail = false,
            Some(l) if inthumbnail => thumbnail.push_str(l),
            _ => {}
        }
    }
    if !thumbnail.is_empty() {
        fs::write(output, &base64::decode(&thumbnail).unwrap()).unwrap();
    }
}
