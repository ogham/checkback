use std::path::{Path, PathBuf};

extern crate quicli;
use quicli::prelude::*;

#[macro_use] extern crate lazy_static;
extern crate walkdir;
extern crate regex;
extern crate reqwest;
use regex::Regex;

extern crate serde;
extern crate serde_json;
use walkdir::{WalkDir, DirEntry};

extern crate datetime;
use datetime::Instant;

extern crate ansi_term;

#[macro_use] extern crate log;
extern crate env_logger;

mod github;
use github::GitHubLink;

mod stacko;
use stacko::StackOverflowLink;

mod ui;
use ui::UI;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Options {

    #[structopt(short = "a")]
    all: bool,

    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,
}


fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    env_logger::init();

    let options = Options::from_args();
    for input_path in options.paths {
        if input_path.is_file() {
            process_file(&input_path, options.all);
        }
        else {
            let walker = WalkDir::new(input_path).into_iter();
            for entry in walker.filter_entry(|e| !is_hidden(e)) {
                let entry = entry.unwrap();
                if entry.file_type().is_file() {
                    process_file(entry.path(), options.all);
                }
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

fn process_file(path: &Path, all: bool) {
    debug!("Scanning file {:?}", path);

    let contents = read_file(path);

    for url in GH.captures_iter(&contents) {
        let link = GitHubLink::get(&url[1], &url[2], url[3].parse().unwrap());
        if all || link.is_recent(*NOW) {
            UI.print_link(path, &link.url, &link.title);
        }
    }

    for url in SO.captures_iter(&contents) {
        let link = StackOverflowLink::get(url[1].parse().unwrap());
        if all || link.is_recent(*NOW) {
            UI.print_link(path, &link.url, &link.title);
        }
    }
}


lazy_static! {
    static ref GH: Regex = Regex::new("https://github.com/([^/]+)/([^/]+)/issues/(\\d+)").unwrap();
    static ref SO: Regex = Regex::new("https://stackoverflow.com/questions/(\\d+)").unwrap();
    static ref NOW: Instant = Instant::now();
}
