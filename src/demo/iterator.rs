fn test() {
    let values = vec![1, 2, 3];
    /// into_iter迭代器：会消耗集合，在每次迭代过程中，集合中的数据会被提供。
    /// 一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    for v in values.into_iter() {
        println!("{}", v)
    }
    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);


    let values = vec![1, 2, 3];
    /// .iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
    ///  在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
    let _values_iter = values.iter();
    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);



    let mut values = vec![1, 2, 3];
    /// 对 values 中的元素进行可变借用, 调用 next 方法返回的类型是 Some(&mut T)
    /// 允许被修改
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        // 需要*解引用
        *v = 0;
    }
    // 输出[0, 2, 3]
    println!("{:?}", values);
    // collector收集
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    // zip是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 Iterator<Item=(ValueFromA, ValueFromB)> 这样的新的迭代器
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}",folks);

}

struct Shoe {
    size: u32,
    style: String,
}

// filter 是迭代器适配器，用于对迭代器中的每个值进行过滤。
// 它使用闭包作为参数，该闭包的参数 s 是来自迭代器中的值，然后使用 s 跟外部环境中的 shoe_size 进行比较，
// 若相等，则在迭代器中保留 s 值，若不相等，则从迭代器中剔除 s 值，最终通过 collect 收集为 Vec<Shoe> 类型。
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

