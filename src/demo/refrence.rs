// 在任意给定时间，要么只能有一个可变的引用，要么只能有多个不可变的引用。
fn tt() {
    let s1 = String::from("hello");
    // &s1, 指向s1的引用，但不拥有它，不拥有这个值，所以当这个引用停止时，s1仍然有效
    // 引用必须总是指向有效的数据，引用无法修改原值，引用只能读取数据，无法修改数据
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s2 = String::from("hello");
    mut_reference(&mut s2);


    let mut s = String::from("hello");

    let r1 = &mut s;
    // error ：cannot borrow `s` as mutable more than once at a time
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

}

// 对象的引用作为参数，而不是值的所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
fn mut_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

// slice
fn slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // 整个字符串
    let slice = &s[0..s.len()];
}

fn first_word(s: &Str) -> &Str {
    let bytes = s.as_bytes();
    bytes
}




