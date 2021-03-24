fn main() {
    // String 类型被分配到堆上，所以能够存储在编译时未知大小的文本；
    // 可以使用 from 函数基于字符串字面值来创建 String；
    let s = String::from("hello");

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}