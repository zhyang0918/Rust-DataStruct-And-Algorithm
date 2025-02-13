enum Person {
    man,
    woman,
}

// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 }
}



fn test() {
    let man = Person::man;
    let woman = Person::woman;
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in(person: Person) -> str {
    match person {
        Person::man => {
            println!("man");
            println!("111")
        },
        Person::woman => {
            println!("woman");
            println!("222")
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// if let 是match的语法糖, 当值匹配时某一模式时执行代码，而忽略其他值
fn if_let_demo() {
    let mut count = 0;
    if let Person::man = Person::man {
        println!("count = {count}");
    } else {
        count += 1;
    }
}