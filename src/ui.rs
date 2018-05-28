use std::path::Path;

use ansi_term::Style;
use ansi_term::Colour::{Blue, Green};
use datetime::{LocalDateTime, ISO};


pub struct UI;

impl UI {
    pub fn print_link(&self, path: &Path, line_number: usize, url: &str, title: &str, last_updated: LocalDateTime) {
        println!("{}:{}:{} {}: last updated on {}",
                 Blue.bold().paint(path.display().to_string()),
                 Green.bold().paint(line_number.to_string()),
                 Blue.underline().paint(url),
                 Style::default().bold().paint(title),
                 last_updated.date().iso());
    }
}
