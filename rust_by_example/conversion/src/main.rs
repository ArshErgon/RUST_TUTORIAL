use std::convert::From;

#[derive(Debug)]
struct Number {
    num: i32,
}

// these are small function
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { num: item }
    }
}

struct Person {
    name: String,
    age: i32,
}

// to make u understand the use impl

impl Person {
    fn intro(&self) {
        println!("My name is: {}", self.name);
    }

    fn tell_age(self) {
        println!("My age is: {}", self.age);
    }
}

fn main() {
    let my_str: &str = "arsh";
    let my_string = String::from(my_str).to_uppercase();
    println!("{my_string} --- {my_str}");

    let my_num = Number::from(20);
    println!("{:?}", my_num);
    println!("{:?}", my_num.num);

    // a program for impl
    let person = Person {
        name: "Arsh".to_string(),
        age: 20,
    };
    person.intro();
    person.tell_age();

    let new_num: i32 = "5".parse().unwrap();
    println!("{new_num}");

    let num_to_str = 5.to_string();
    println!("{num_to_str}");

    // unpacking a string number in a integer
    let new_num_without_type = "10".parse::<i32>().unwrap();
    println!("{}", new_num_without_type);
    println!("{}", new_num + new_num_without_type);
}
