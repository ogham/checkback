use std::path::Path;

pub use ansi_term::Style;
pub use ansi_term::Colour::{Blue, Green};


pub struct UI;

impl UI {
    pub fn print_link(&self, path: &Path, line_number: usize, url: &str, title: &str) {
        println!("{}:{}:{} {} has been updated recently",
                 Blue.bold().paint(path.display().to_string()),
                 Green.bold().paint(line_number.to_string()),
                 Blue.underline().paint(url),
                 Style::default().bold().paint(title));
    }
}
