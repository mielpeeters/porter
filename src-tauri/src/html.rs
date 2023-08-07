/*!
  Handle html strings for the porter project
*/

use csv::Reader;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug)]
pub enum Error {
    // no match found
    NoMatch,
}

pub fn open_template(template: PathBuf) -> String {
    let mut html = String::new();

    let template = fs::read_to_string(template).unwrap();

    html.push_str(template.as_str());

    html
}

pub fn insert(html: &mut String, entries: &[String], pattern: &str) -> Result<(), Error> {
    for entry in entries {
        let Some(location) = html.find(pattern) else {
            return Err(Error::NoMatch);
        };

        // surround with newlines to result in nicer fotmatting
        let entry = format!("{}\n", entry);
        html.insert_str(location, &entry);
    }
    self::clean(html, pattern);
    Ok(())
}

/// clean the pattern from the inputted html.
pub fn clean(html: &mut String, pattern: &str) {
    let start = html.find(pattern).unwrap();
    let end = start + pattern.len();
    html.replace_range(start..end, "");
}

pub fn save(html: &String, file: PathBuf) {
    let mut buffer = File::create(file).unwrap();

    let _ = buffer.write_all(html.as_bytes());
}

pub fn open_csv(path: PathBuf) {
    let mut reader = Reader::from_path(path).unwrap();
    show_csv(&mut reader);
}

fn show_csv(csv: &mut Reader<File>) {
    let header = csv.headers().unwrap();

    for row in header.iter() {
        print!("{row}, ");
    }

    println!();

    for record in csv.records() {
        if let Ok(record) = record {
            for row in record.iter() {
                print!("{row}, ");
            }
            println!();
        } else {
            println!("didn't work out");
        }
    }
}
