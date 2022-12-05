use std::io;

fn main() {
    // let mut x = 10;
    // println!("immutable x {x}");
    // x += 1;
    // println!("mutable x {x}");

    // const FIVE_HOUR_IN_SECONDS: i64 = (60 * 60 * 5) / 2;
    // println!("PI {FIVE_HOUR_IN_SECONDS}");

    // Showding a variable in rust

    // let y = 10;
    // these curly brackets are inner scope
    // {
    // let y = y * 90;
    // println!("Inside a inner scope: {y}");
    // and the variables
    // defined inside here cant be used
    // outside these brackets
    // }

    // rust counts the whitespaces also
    // use trim() to ignore the whitespaces
    // let space_variable = "   ";
    // println!("the untrimed string length: {}", space_variable.len());
    // println!("the length of trimed variable: {}", space_variable.trim().len());

    // println!("{y}");

    // tuples
    // getting the values in tuples is by doing
    // tup.0  # its the index of the element

    // let tup: (i32, u64, f32) = (-12, 890, 29.2);
    // println!("{}", tup.0);

    // let (x, y, z) = tup;
    // println!("{x} : {y} : {z}");

    // Arrays in RUST
    // array in rust have fixed size, array arent dynamic
    // once the length is defined it will be fixed all
    // the way to the program end
    // to change the value inside the
    // array use mut while defining the array

    // let mut new_array: [i32; 10] = [1, 2, 3, 4, 5, 0 ,0, 0, 0, 0];
    // println!("{:?} :", new_array);

    // new_array[6] = 10;

    // println!("{:?} :", new_array);

    // like python: array = [0] * 4
    // [where the first option is the num to be filled; length]

    // let new_array_second = [3; 4];
    // println!("{:?}", new_array_second);

    // make a program to take the
    // size of an array and fill it with the number
    // filled by the user

    println!("Enter the number: ");
    // let mut array_size = String::new();

    // getting the size of the array to be made

    let mut new_array_last = [0; 5];

    let mut idx: i32 = 0;
    println!("Will gonna loop {}", new_array_last.len());

    let size: i32 = new_array_last.len().try_into().unwrap();
    println!("{size}");

    while idx < size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Not a valid num");

        let num: i32 = num.trim().parse().expect("not valid number");

        new_array_last[idx as usize] = num;
        idx += 1;
        println!("{idx}");
    }

    println!("{:?}", new_array_last);
}
