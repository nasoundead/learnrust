use std::rc::Rc;

fn main() {
    let r = Rc::new(123);
    let r2 = r.clone();
    println!("r = {:?}", r);
    println!("r22 = {:?}", r2);
}
