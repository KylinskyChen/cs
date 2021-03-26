#[derive(Debug)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut use2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        ..user1
    };
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);
    
    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn first_word(s: &String) -> usize {
    // 用 as_bytes 方法将 String 转化为字节数组；
    let bytes = s.as_bytes();

    // 使用 iter 方法在字节数组上创建一个迭代器；
    // enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回；
    // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用；
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}