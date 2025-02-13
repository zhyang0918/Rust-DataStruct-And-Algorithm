use std::fs::File;
use std::{fs, io};


fn test() {
    let ss = read().expect("TODO: panic message");
    println!("{}", ss);

    let s = read_username_from_file();
    match s {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}

fn read() -> Result<String, io::Error> {
    // 将会把 Ok 中的值返回给变量 f。如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者。同理也适用于 read_to_string 调用结尾的 ?。
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}