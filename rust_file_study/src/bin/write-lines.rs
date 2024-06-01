use std::{fs::File, io::Write};

fn main() {
    let mut file = File::create("a.txt").unwrap();
    for line in Lines::new().take(10) {
	file.write_all(line.as_bytes()).unwrap();
    }
}

struct Lines(usize);

impl Lines {
    fn new() -> Lines {
	Lines(0)
    }
}

impl Iterator for Lines {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
	let n = 1000000000 + self.0;
	let line = format!("{}\n", n);
	self.0 = self.0 + 1;
	Some(line)
    }
}
