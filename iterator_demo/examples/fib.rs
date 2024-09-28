struct Fib {
    a: i32,
    b: i32,
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.a;
        self.a = self.b;
        self.b = res + self.b;
        Some(res)
    }
}

fn main() {
    let fib = Fib { a: 1, b: 1 };
    for i in fib.take(10) {
        println!("{}", i);
    }
}