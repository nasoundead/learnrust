use std::io::Read;
use std::{net::TcpStream, io::Write};
use std::str;
fn main() {

    let mut stream = TcpStream::connect("localhost:8000").unwrap();
    stream.write("hello".as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Response from server: {:?}", str::from_utf8(&buffer));
}
