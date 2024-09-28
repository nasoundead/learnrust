use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let file = File::open("a.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
	let line = line.unwrap();
	println!("{}", line);
    }
}
