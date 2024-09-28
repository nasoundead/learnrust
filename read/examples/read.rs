use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f: File = File::open("foo.txt")?;
    let mut buffer: [u8; 10] = [0; 10];

    // read up to 10 bytes
    loop {
	let size = f.read(&mut buffer)?;
	println!("{:?}, size:{}", buffer, size);
	if size < 10 {
	    for i in size..10 {
		print!("{i}");
		// buffer[i] = 0;
	    }
	    break;
	}
    }

    println!("{:?}", buffer);
    // let mut buffer: Vec<u8> = Vec::new();
    // // read the whole file
    // f.read_to_end(&mut buffer)?;

    // // read into a String, so that you don't need to do the conversion.
    // let mut buffer: String = String::new();
    // f.read_to_string(&mut buffer)?;

    // and more! See the other methods for more details.
    Ok(())
}
