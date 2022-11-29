// fn read_iter(file_name: &str, func: fn(&str)) {
//     let file = File::open(file_name).expect("file not found!");
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         func(&line.unwrap());
//     }
// }
// use std::fs::File;
// use std::io::{self, BufRead};
// use std::io::prelude::*;
// use std::path::Path;
use std::error::error;
use std::env;
use csv::writer;
use serde::serialize

fn main() {
    let mut wtr = csv::Writer::from_path("data.csv").unwrap();
    wtr.write_record(&"data.csv").unwrap();
    wtr.flush().unwrap();

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open("data")?;
    Ok(io::BufReader::new(file).lines())
}
pub fn append (file_name: &str, data: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
//reading file from list
fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

