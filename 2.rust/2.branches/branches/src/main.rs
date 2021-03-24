fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }

    if number != 0 {
        println!("number was something other than zero.");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    loop {
        println!("again!");
        break;
    }

    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("the value is : {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
