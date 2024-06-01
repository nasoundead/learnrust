fn main() {
    let mut x = 5;
    {
	let y = &mut x;
	*y += 1; // 这里必须加上*
	println!("{}", y);
    }
    println!("{}", x);
}
