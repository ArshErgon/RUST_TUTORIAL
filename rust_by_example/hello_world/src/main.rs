fn main() {
    println!("Hello, world!");

    println!("{}", 12);

    println!("{0} {1} {0}", 1, 2);

    println!(
        "{first} {second} {third}",
        first = "this will be first",
        second = "this will be second",
        third = "this will be third"
    );

    // different type of formatting

    println!("{:b}", 10); // binary
    println!("{:o}", 15); // oct
    println!("{:x}", 15); // hexa
    println!("{:X}", 15); // hexa

    // making a binary or any other number system with format!
    let bin_num = format!("{:b}", 10).to_string();

    println!("{bin_num}");

    // give it spaces from left 5
    println!("{num:>5}", num = 1);

    // $ whenever used remember the above example and remove the  with width
    println!("{num:>width$}", num = 10, width = 20);
    // {:?} for debug
    // {} for display
}
