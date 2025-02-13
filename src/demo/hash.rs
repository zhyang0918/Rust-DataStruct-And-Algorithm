use std::collections::HashMap;

fn create() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn iter() {
    // 使用元组创建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");
    // 取值
    let score = scores.get(&team_name);
    // 另一种方式的遍历：
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 只在没有键的情况下插入：
    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Yellow")).or_insert(300);
}

fn demo() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 迭代map
    for word in text.split_whitespace() {
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        // *解引用后，才能赋值
        *count += 1;
    }

    println!("{:?}", map);
}

fn hash_ownership() {
    // string没有实现copy trait， 拥有所有权，借给hashmap后，hashmap会成为这些值的所有者，原先的string不再有效
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
}