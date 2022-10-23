use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    // 产生神秘数字
    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("secret num: {}", secret_num);

    loop {
        println!("input you guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("can not read line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        println!("your guess num is: {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you are right");
                break;
            },
        }
    }
    
}
