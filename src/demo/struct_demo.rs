struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn test() {
    // 整个实例必须是可变的
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        // email成员名与变量名一样，所以可以简写
        email,
        // username成员名与变量名一样，所以可以简写
        username,
        active: true,
        sign_in_count: 1,
    }
}