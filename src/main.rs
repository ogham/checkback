use std::path::Path;

#[macro_use] extern crate quicli;

#[macro_use] extern crate lazy_static;
extern crate walkdir;
extern crate regex;
extern crate reqwest;
use regex::Regex;

extern crate serde;
extern crate serde_json;
use walkdir::WalkDir;

extern crate datetime;
use datetime::Instant;

extern crate ansi_term;

mod github;
use github::GitHubLink;

mod stacko;
use stacko::StackOverflowLink;

mod options;
use options::input_paths;

mod ui;
use ui::UI;


fn main() {
    for input_path in input_paths() {
        for entry in WalkDir::new(input_path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let contents = read_file(entry.path());
                process_file(entry.path(), &contents);
            }
        }
    }
}

fn read_file(path: &Path) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn process_file(path: &Path, contents: &str) {
    for url in GH.captures_iter(&contents) {
        let link = GitHubLink::get(&url[1], &url[2], url[3].parse().unwrap());
        if link.is_recent(*NOW) {
            UI.print_link(path, &link.url, &link.title);
        }
    }

    for url in SO.captures_iter(&contents) {
        let link = StackOverflowLink::get(url[1].parse().unwrap());
        if link.is_recent(*NOW) {
            UI.print_link(path, &link.url, &link.title);
        }
    }
}


lazy_static! {
    static ref GH: Regex = Regex::new("https://github.com/([^/]+)/([^/]+)/issues/(\\d+)").unwrap();
    static ref SO: Regex = Regex::new("https://stackoverflow.com/questions/(\\d+)").unwrap();
    static ref NOW: Instant = Instant::now();
}
