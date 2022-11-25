// enum IpAddress {
//     One(String),
//     Two(bool),
//     Three(String, i32),
//     Four { x: i32, y: i32 },
// }

// Rust dont have NULL
// rust use Option<T>
// Here's the example

// enum Option<T> {
//     None,
//     Some(T),
// }

// using enum with the match control flow

// enum Number {
//     One,
//     Two,
//     Three,
// }

// fn integer_to_string(integer: Number) -> u8 {
//     match integer {
//         Number::One => 1,
//         Number::Two => 2,
//         Number::Three => 3,
//     }
// }

fn main() {
    // enum <T> program

    // let some_number = Option::Some(String::from("ARSH"));
    // println!("{:?}", some_number);
    // let number: Option<i32> = Option::None;
    // println!("{:?}", number);

    // using enum with the match control flow

    // println!("{}", integer_to_string(Number::Two));

    // let x = IpAddress::One(String::from("Arsh"));
    // let y = IpAddress::Two(true);
    // let z = IpAddress::Three(String::from("Ergon"), 90);
    // let m = IpAddress::Four { x: 20, y: 230 };
    // println!("{:?} {:?} {:?} {:?}", x, y, z, m);

    // if let control flow

    let max_num = Some(3u8);
    if let Some(max) = max_num {
        println!("The maximum number of {:?} --- {max}", max_num);
    } else {
        println!("this not work");
    }
}
