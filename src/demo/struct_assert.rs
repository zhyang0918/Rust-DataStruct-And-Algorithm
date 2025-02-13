// 自定义的struct，enum 如果想使用assert_eq!， assert_ne!宏，需要实现PartialEq trait， 使用derive宏：#[derive(PartialEq, Debug)]
#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test() {

}