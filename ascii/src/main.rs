fn main() {
    // 遍历所有的字符
    for i in 0..256 {
        // 将字符转换为ASCII码
        let char_value = i as u8 as char;

        // 打印字符和它的ASCII码
        println!("{:03} - 0x{:02X} - \"{}\"", i, i, char_value);
    }
}
