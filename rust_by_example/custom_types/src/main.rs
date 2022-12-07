// making stuct

struct Person {
    name: String,
    num: i32,
}

// activity find area of rectangle using struct

struct RectangleArea {
    length: f64,
    breadth: f64,
}

fn to_find_area(l: f64, b: f64) -> f64 {
    return l * b;
}

// the uses of enums
#[derive(Debug)]
#[allow(dead_code)]

enum Detail {
    Name { f_name: String, l_name: String },
}

// different enum program

#[allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

enum Job {
    Yes,
    No,
}

fn main() {
    let new_person = Person {
        name: String::from("Arsh"),
        num: 9,
    };
    println!(
        "Name:-> {0}\nNumber:-> {1}",
        new_person.name, new_person.num
    );
    let area = RectangleArea {
        length: 10.0,
        breadth: 5.0,
    };
    println!("{}", to_find_area(area.length, area.breadth));

    // enums

    let user_name = Detail::Name {
        f_name: String::from("Arsh"),
        l_name: String::from("Ali"),
    };
    println!("{:#?}", user_name);

    use crate::Job::*;
    use crate::Status::{Poor, Rich};

    let status = Rich;
    let job = Yes;

    match status {
        Rich => println!("Rich works"),
        Poor => println!("Poor dont"),
    }

    match job {
        Yes => println!("YES"),
        No => println!("No"),
    }
}
