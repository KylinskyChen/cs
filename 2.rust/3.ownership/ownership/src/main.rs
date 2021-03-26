fn main() {
    let s1 = String::from("hello");

    // & 符号就是 引用，它们允许你使用值但不获取其所有权；
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");
    
    change(&mut s);

    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);
}

// 在特定作用域中的特定数据只能有一个可变引用；
fn change(someString: &mut String) {
    someString.push_str(", world!");
}

// 使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*；
// &String s 指向 String s1；
// 获取引用作为函数参数称为 借用（borrowing）；
// （默认）不允许修改引用的值；
fn calculate_length(s: &String) -> usize {
    s.len()
}