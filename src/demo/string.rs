#[allow(dead_code)]
struct Circle {
    radius: i32
}

/// 要把任何类型转换成 String，只需要实现那个类型的 ToString trait。然而不要直接这么做，
/// 您应该实现fmt::Display trait，它会自动提供 ToString，并且还可以用来打印类型，就像 print!那样
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn test() {
    // from创建
    let s = String::from("hello");
    // to_string方式创建
    let s = "hello".to_string();
    // 可变字符串
    let mut s = String::from("foo");
    // 字符串slice的内容追加到string
    s.push_str("bar");
    // 使用push，追加一个字符
    let mut s = String::from("lo");
    s.push('l');
    // 使用+操作符拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // 使用format宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // rust中String不支持索引访问 error： s[0]
    // 字符串遍历
    for c in s.chars() {
        println!("{}", c);
    }
    // 转字符串
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // 解析字符串
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
