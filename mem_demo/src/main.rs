use std::mem::{size_of, align_of};



fn main() {
    let a = size_of::<i32>();
    println!("{a}");

    let b = size_of::<char>();
    println!("{b}");

    let c = size_of::<(char, u8, i32)>();
    println!("{c}");

    // (char, u8, i32)的对齐属性是4
    let d = align_of::<(char, u8, i32)>();
    println!("{d}");
}
