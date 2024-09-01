use std::{rc::Rc, vec};

fn main() {
    let r = Rc::new(123);
    let r2 = r.clone();
    println!("r : {:?}", r);
    println!("r2: {:?}", r2);

    let r3 = r.clone();
    println!("r3: {:?}", r3);

    let r4 = r.clone();
    println!("r4: {:?}", r4);

    let r5 = r.clone();
    println!("r5: {:?}", r5);
}
