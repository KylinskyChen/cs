// 将 io（输入/输出）库引入当前作用域；
// io 库来自于标准库（也被称为 std）；
use std::io;

// Rng 是一个 trait，它定义了随机数生成器应实现的方法；
// 想使用这些方法的话，此 trait 必须在作用域中；
use rand::Rng;

use std::cmp::Ordering;

// 运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开；

fn main() {
    println!("Guess the number!");

    // 提供实际使用的随机数生成器；
    // 位于当前执行线程的本地环境中，并从操作系统获取 seed；
    // 取两个数字作为参数，并生成一个范围在两者之间的随机数；
    // 它包含下限但不包含上限，所以需要指定 1 和 101 来请求一个 1 和 100 之间的数；
    let secretNumber = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secretNumber);

    // loop 关键字创建了一个无限循环；
    loop {
    
        println!("Please input your guess.");

        // 创建一个储存用户输入的地方；
        // new 是 String 类型的一个 关联函数（associated function）；
        // 静态方法（static method）；
        // new 函数创建了一个新的空字符串，你会发现很多类型上有 new 函数，因为它是创建类型实例的惯用函数名；
        let mut guess = String::new();  // 可变；
        // let foo = 5;                 // 不可变；

        // std::io::stdin；
        // read_line 的工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数；
        // 这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去；
        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝；
        io::stdin().read_line(&mut guess)
            // 逻辑行；
            // 通过换行加缩进来把长行拆开是明智的；
            .expect("Faild to read line");
        
        // Rust 标准库中有很多叫做 Result 的类型：一个通用的 Result 以及在子模块中的特化版本，比如 io::Result；
        // Result 类型是 枚举（enumerations），通常也写作 enums；
        // 枚举类型持有固定集合的值，这些值被称为枚举的 成员（variants）；
        // Result 的成员是 Ok 和 Err；
        // 这些 Result 类型的作用是编码错误处理信息；
        // io::Result 的实例拥有 expect 方法；
        // 如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息;
        // 如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果；
        // 如果不调用 expect，程序也能编译，不过会出现一个警告；
        // Rust 警告我们没有使用 read_line 的返回值 Result，说明有一个可能的错误没有处理；
        // 消除警告的正确做法是实际编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 expect；
        
        // Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值；
        // 许我们复用 guess 变量的名字，而不是被迫创建两个不同变量；
        // String 实例的 trim 方法会去除字符串开头和结尾的空白字符；
        // trim 方法消除 \n；
        // 字符串的 parse 方法 将字符串解析成数字；
        // oarse 方法可以解析多种数字类型，因此需要告诉 Rust 具体的数字类型，这里通过 let guess: u32 指定；
        // 后面的冒号（:）告诉 Rust 我们指定了变量的类型；
        let guess: u32 = match guess.trim().parse() {
            // 如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok；
            // 这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num，最后如愿变成新创建的 guess 变量；
            Ok(num) => num,
            // _ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息；
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Rust 标准库中尚未包含随机数功能；
        // crate 是一个 Rust 代码包。我们正在构建的项目是一个 二进制 crate，它生成一个可执行文件；
        // rand crate 是一个 库 crate，库 crate 可以包含任意能被其他程序使用的代码；
        // 使用 rand 编写代码之前，需要修改 Cargo.toml 文件，引入一个 rand 依赖；

        // cmp 方法用来比较两个值并可以在任何可比较的值上调用；
        // 它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较；
        // 返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员；
        // 使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么；

        // match 表达式由 分支（arms） 构成；
        // 一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码；
        // Rust 获取提供给 match 的值并挨个检查每个分支的模式；
        match guess.cmp(&secretNumber) {
            // Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal；
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }

    // Rust 有一个静态强类型系统，同时也有类型推断；
    // 例如：当我们写出 let guess = String::new() 时，Rust 推断出 guess 应该是 String 类型，并不需要我们写出类型；

}