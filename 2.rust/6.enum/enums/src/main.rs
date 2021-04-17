enum IpAddrKind{
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8);
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_type: IpAddrKind) {

}

// <T> 意味着 Option 枚举的 Some 成员可以包含任意类型的数据；
let some_number = Some(5);
let some_string = Some("a string");

// 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型；
let absent_number: Option<i32> = None;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}




let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);


