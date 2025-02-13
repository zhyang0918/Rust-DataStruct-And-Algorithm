#[cfg(test)]
fn vec() {
    // 显示定义类型的创建
    let v: Vec<i32> = Vec::new();
    // vec!宏方式创建
    let v = vec![1, 2, 3];

    // 使用&和[]返回一个引用
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 使用get, 返回一个Option<&T>索引下标从0开始, 使用get数组越界时，返回None
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 遍历
    for i in &v {
        println!("{}", i);
    }

    // 可变引用遍历
    for i in &mut v {
        *i += 50;
    }

    // 存储不同的值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
