// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }
// fn main() {

//     let mut args = std::env::args();
//     // println!("args: {:?}", args);
//     let pattern = args.nth(1).expect("no pattern provided");
//     let path = args.nth(2).expect("no path provided");
//     let args = Cli {
//         pattern: pattern,
//         path: std::path::PathBuf::from(path),
//     };
// }

use Clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
