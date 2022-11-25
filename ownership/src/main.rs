fn main() {
    // remember if you want give a value to another variable
    // like a = 10; b = a;
    // remember you can 'b' not 'a' as Rust dont follow the rules of deepcopy or shallowcopy
    // use .clone() instead

    // let name = "Arsh";
    // let new_name = name.clone();
    // println!("{new_name} real variable: {name}");

    // multi funcation return

    // let unknow_leng_str = String::from("Arsh");
    // let (string_, size) = calculate_the_leng_of_str(unknow_leng_str);
    // println!("{string_} the length of : {size}.");

    // Program on ownership well defined.

    // let mut name = String::from("Arsh ");
    // passing_the_ownership(&mut name);
    // print!("{name}");

    // let name = String::from("Arsh");

    // println!("{name}");

    // slicing name &variable_name[..]

    // let name = String::from("Arsh");
    // println!("{}", &name[..]);

    // assert_eq!(); to check (this == to_this)

    // assert_eq!(name, &name[0..2]);




}

// fn passing_the_ownership(s: &mut String) {
//     s.push_str("Ergon");

// }

// multi line function:

// fn calculate_the_leng_of_str(string_: String) -> (String, usize) {
//     return (string_.clone(), string_.len());
// }
