use std::{net::TcpListener, io::{Read, Write}};

fn main() {
    let listener = TcpListener::bind("localhost:8000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
    }   
}
