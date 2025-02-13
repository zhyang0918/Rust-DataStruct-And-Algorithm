#[allow(dead_code)]
mod algo;

use rand::Rng;
use std::cmp::Ordering;
use std::io::Read;
use std::{io, mem};
use std::fmt;

// 常量
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// 行注释
/*...块注释*/
/// 文档行注释
/** `add_two` 将指定值加2

```
let arg = 5;
let answer = my_crate::add_two(arg);

assert_eq!(7, answer);
```
*/
fn add_two(x: i32) -> i32 {
    x + 2
}


fn main() {
    // variables();
    // loop_test();
    // print()
}

fn print() {
    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);
    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number=1, width=6);
}

// 定义结构体
struct Structure(i32);


//自定义结构体实现Display
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0);
        // 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
        // 否则（没有出错）继续执行后面的语句。
        Ok(write!(f, "{}", self.0)?)
    }
}

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;
        write!(f, "[")?;
        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}



// 变量
fn variables() {
    let cc = 1;
    // error 变量定义默认是不可变的
    // cc = 2;
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
    let f32: f32 = 1.0;
    // 复合类型，元组，数组 元组可以有不同类型的成员， 数组成员只有一种类型
    // 可以充当函数的参数和返回值
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 解构，从而将绑定的值给变量， tup 值给x,y,z
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
    // 下标从 0 开始
    println!("first element of the array: {}", aa[0]);
    println!("second element of the array: {}", aa[1]);
    // `len` 返回数组的大小
    println!("array size: {}", aa.len());
    // 数组是在栈中分配的
    println!("array occupies {} bytes", size_of_val(&aa));

    // 向量，可变长度的集合
    let vec = [1, 2, 3, 4, 5];
    // 表达式赋值
    let y = {
        let x = 3;
        // 注意最后一句，没有; 如果有; 它就转化成语句了，语句不会有返回值
        x + 1
    };
}

fn scope() {
    // 此绑定生存于 scope 函数中
    let i = 1;
    // 这是一个代码块, 只存在于scope中
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        // 此绑定*遮蔽*了外面的绑定
        let i = 5_f32;
        println!("inner long: {}", i);
    }

    println!("outer long: {}", i);
    // 此绑定同样*遮蔽*了前面的绑定
    let i = 'a';
    println!("outer long: {}", i);
}



// 单元结构体
struct Unit;
// 元组结构体
struct Pair(i32, f32);


fn structFn() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    // 以 Debug 方式打印结构体
    println!("{:?}", peter);
    // 实例化结构体 `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);
    // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rle {
        // 结构体的实例化也是一个表达式
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    // 实例化一个单元结构体
    let _unit = Unit;
    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);
    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}


// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。 结构体嵌套
    top_left: Point,
    bottom_right: Point,
}




// 返回值函数
fn plus_test(x: i32) -> i32 {
    // 不能有; 有;就转化成语句了，语句不会有返回值
    x + 1
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
            // 跳过这次迭代剩下的部分
            continue;
        }

        if counter == 20 {
            // 跳出整个循环   counter * 2: loop表达式的返回值
            break counter * 2;
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
