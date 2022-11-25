// basic calling of the function

// fn add() {
//     println!("Addition");
// }

// fn main() {
//     println!("Hello, world!");
//     add();
// }

// things to remember:
// println!() is used for getting a return value if a function have one
// the print!() is used when the function doesnt have a return type, or we dont want to see the return type value.
// Rust is statically type lan, means it required a parameter to be defined what it data will be pushed inside it,
// and should defined the return the type data_type also, if required.

// fn add(num_one: i32, num_two: i32) -> i32 {
//     return num_one + num_two;
// }

// fn add_the_two_string(string_one: String, string_two: String) -> String {
//     println!("{string_one} {string_two}");
//     return "String".to_string();
// }


// getting values in a variable from a function

// fn value() -> i32 {
//     return 10;
// }




fn main() {
    // function s are defined here

    // println!("{}", add(10, 20));
    // print!("{:?}", add_the_two_string("arsh".to_string(), "ergon".to_string()))

    // look closly the x is not like x += 1 we are not re-assigning the variable x 
    // do remember recursion in this time. 
    // the value of y is only going to assign to it only if the inner block finished complelety.

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The value of y is: {y}");

    // getting the value from the function

    // let value_from_fun = value();
    // println!("{value_from_fun}");

}
