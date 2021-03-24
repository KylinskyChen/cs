fn main() {
    // 使用大型数据结构时，适当地使用可变变量，可能比复制和返回新分配的实例更快；
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 声明常量使用 const 关键字而不是 let；
    // 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值；
    // Rust 常量的命名规范是使用下划线分隔的大写字母单词，并且可以在数字字面值中插入下划线来提升可读性；
    const MAX_POINTS: u32 = 100_000;

    // 我们可以定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量；
    // 为第一个变量被第二个 隐藏 了，这意味着使用这个变量时会看到第二个值；
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    // 隐藏与将变量标记为 mut 是有区别的；
    // 1. 当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不变的；
    // 2. 当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字；
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    // 隐藏使我们不必使用不同的名字；
    // 如果尝试使用 mut，将会得到一个编译时错误，不能改变变量的类型；
    // let mut spaces = "   ";
    // spaces = spaces.len();  

    // 使用 parse 将 String 转换为数字时，必须增加类型注解；
    let guess: u32 = "42".parse().expect("Not a number!");

    // 浮点数采用 IEEE-754 标准表示；
    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'Z';
    // 在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值；
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构（destructuring），模式匹配解构；
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小；
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];

    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    }

    let x = five()
    let x = plus_one(5);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}