#[allow(dead_code)]
mod algo;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

// 常量
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;


fn main() {
    // variables();
    loop_test();
}

// 变量
fn variables() {
    let mut x = 5;
    x = 6;
    println!("x is {}", x);

    let mut space = "    ";
    // 不允许更改变量的类型
    // space = space.len();

    // let 关键字有效的创建一个新的变量
    let cc = 5;
    // 复用cc来遮蔽之前的cc
    let cc = cc + 1;

    {
        // 遮蔽前面的cc
        let cc = cc * 2;
        // 12
        println!("The value of cc in the inner scope is: {}", cc);
    }
    // 5+1 = 6
    println!("The value of cc is: {}", cc);

    // 默认i32
    let i = 5;
    // boolean
    let b: bool = true;
    // char
    let c = 'c';
    // float 默认64
    let f64 = 1.0;
    let f32 :f32 = 1.0;
    // 复合类型，元组，数组 元组可以有不同类型的成员， 数组成员只有一种类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构
    let (x, y, z) = tup;
    println!("x is {}, y is {}, z is {}", x, y, z);
    let xx: (i32, f64, u8) = (500, 6.4, 1);
    // 另一种解构
    let one = xx.0;
    let two = xx.1;
    let three = xx.2;
    // 数组, 不可变长度的集合
    let a = [1, 2, 3, 4, 5];
    // 数组， 5个3
    let aa = [3; 5];
    // 向量，可变长度的集合
    let vec = [1, 2, 3, 4, 5];
    // 表达式赋值
    let y = {
        let x = 3;
        // 注意最后一句，没有; 如果有; 它就转化成语句了，语句不会有返回值
        x + 1
    };
}

// 返回值函数
fn plus_test(x: i32) -> i32 {
    // 不能有; 有;就转化成语句了，语句不会有返回值
    x+1
}

// if 表达式赋值
fn if_test() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number)
}

// loop 循环
fn loop_test() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    print!("The result is {}", result)
}

// for 循环
fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    // for循环反转
    for number in (1..4).rev() {
        println!("{}!", number)
    }
}







fn main1() {
    println!("welcome to rust data struct and algorithm");
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        // 换成OK，err保证程序不崩溃
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
