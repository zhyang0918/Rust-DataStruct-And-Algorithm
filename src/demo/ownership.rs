// rust中每个值都有一个默认的所有者变量。在任意时刻，最多只能有一个所有者。当所有者离开作用域，值被释放。
fn ownership() {
    let s1 = String::from("hello");
    s1.push_str(", world");
    // s2也指向s1指向的地址，不过产生了个问题，s1,s2都离开作用域的话，会尝试释放相同的内存，会产生二次释放
    // rust为了解决这个问题，使用move语义，将s1的所有权移动到s2中，s1不再指向s1指向的地址，s1变成无效值，当s2离开作用域，s2释放内存，完毕
    // 字符串类型没有copy trait， 所以s2 = s1后，s1失效
    let s2 = s1;
    // s2的值进入函数里
    take_ownership(s2);
    // s2离开作用域，失效
    let x = 5;
    // x 应该移动函数里,
    makes_copy(x);
    // 但 i32 是 Copy 的，所以在后面可继续使用 x




}

fn take_ownership(param: String) {
    println!("{}", param)
}

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作