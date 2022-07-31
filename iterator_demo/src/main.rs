fn main() {
    let v = [1i32, 2, 3, 10, 4];
    let add = |a, b| a + b;
    assert_eq!(v.iter().fold(0, add), 20); // legal
                                           // assert_eq!(v.iter().reduce(add).unwrap(), 20); // illegal
}

#[cfg(test)]
mod tests {
    #[test]
    fn inspect() {
        let v = vec![-1, 2, -3, 4, 5].into_iter();

        let _positive_numbers: Vec<i32> = v
            .inspect(|x| println!("Before filter:{:?}", x))
            .filter(|x: &i32| x.is_positive())
            .inspect(|x| println!("After filter:{:?}", x))
            .collect();

        println!("After filter:{:?}", _positive_numbers);
    }

    #[test]
    fn map() {
        let v = vec!["hello", "world"];
        let w: Vec<String> = v.into_iter()
        .map(String::from)
        .collect();
        println!("After map:{:?}", w);
    }

    // filter_map 有点像 map 和 filter 的组合。它在处理 Option 而不是 bool 的时候有优势：
    #[test]
    fn filter_map() {
        let v = vec!["hello", "world", "!"];

        let w: Vec<String> = v.into_iter()
            .filter_map(|x| {
                if x.len() > 2 {
                    Some(String::from(x))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(w, vec!["hello".to_string(), "world".to_string()]);
    }

    // chain 用于合并两个迭代器：
    #[test]
    fn chain() {
        let x = vec![1, 2, 3, 4, 5].into_iter();
        let y = vec![6, 7, 8, 9, 10].into_iter();

        let z: Vec<u64> = x.chain(y).collect();
        assert_eq!(z.len(),10);
    }

}
