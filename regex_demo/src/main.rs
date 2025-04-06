use regex::Regex;

fn main() {
    let text = "hello 123 world 456";
    let re = Regex::new(r"(\w+)\s*(\d+)").unwrap();

    for cap in re.captures_iter(text) {
        for i in 0..cap.len() {
            println!("Match group {}: {}", i, cap.get(i).unwrap().as_str());
        }
    }

    let vec = re
        .captures_iter(text)
        .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
        .collect::<Vec<_>>();
    for (word, num) in vec {
        println!("Word: {}, Number: {}", word, num);
    }
}
