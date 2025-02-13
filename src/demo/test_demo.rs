// #[cfg(test)]: 告诉编译器，只在执行cargo test时才编译和运行测试代码，而在运行cargo build时，忽略测试代码。
// #[test] 也是
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}