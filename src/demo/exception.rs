use std::fs::File;
use std::io::{ErrorKind, Read};

fn demo() {
    // 类似 java的try catch
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 简写：
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 将会把 Ok 中的值返回给变量 f。如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者。同理也适用于 read_to_string 调用结尾的 ?。
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
