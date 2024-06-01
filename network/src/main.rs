// #![feature(lookup_host)]
// use std::net::lookup_host;

fn main() {
    // let args: Vec<_> = env::args().collect();
    // if args.len() != 2 {
    //	eprintln!("Please provide only one host name");
    //	std::process::exit(1);
    // } else {
    //	println!("First argument is {}", args[0]);
    //	println!("Second argument is {}", args[1]);
    // }
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    println!("Digest is : {:x}", digest);
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}
