#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Point({}, {}) dropped", self.x, self.y);
    }
}
fn main() {
    let p = &Point { x: 0, y: 0 };

    println!("{:?}", p);
    println!("{p:#?}");

    let a_vec : Vec<String> = vec![];
    if a_vec.is_empty() {
        println!("nothing in there!");
    }
}

#[cfg(test)]
mod tests {
    // Moves all the elements of other into Self, leaving other empty.
    // From your example, the following code will concatenate two vectors by mutating a and b:
    #[test]
    fn test_append() {
        let mut a = vec![1, 2, 3];
        let mut b = vec![4, 5, 6];

        a.append(&mut b);
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
        assert_eq!(b, []);
    }

    #[test]
    fn test_extend() {
        let mut a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        a.extend(b);
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
        // b is moved can't be used anymore
        // assert_eq!(b, [4, 5, 6]);
    }

    // Note that the vector b is moved instead of emptied.
    // If your vectors contain elements that implement Copy,
    // you can pass an immutable reference to one vector to extend() instead in order to avoid the move.
    // In that case the vector b is not changed:
    #[test]
    fn test_extend_ref() {
        let mut a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        a.extend(&b);
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
    }

    #[test]
    fn test_chain_consumed() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        let c: Vec<i32> = a.into_iter().chain(b.into_iter()).collect(); // Consumed
        // println!("Consumed {:?}", a);
        assert_eq!(c, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_chain_referenced() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        let mut c: Vec<&i32> = a.iter().chain(b.iter()).collect(); // Referenced
        assert_eq!(c, [&1, &2, &3, &4, &5, &6]);
        c[0] = &100;
        println!("a: {a:?}");
        println!("c: {c:?}");
    }

    #[test]
    fn test_chain_cloned() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        let c: Vec<i32> = a.iter().cloned().chain(b.iter().cloned()).collect(); // Cloned
        assert_eq!(c, [1, 2, 3, 4, 5, 6]);
    }
}
