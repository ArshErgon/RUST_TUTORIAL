fn main() {
    let mut change = 10;
    change += 1;
    println!("{change}");

    // short life variable

    let new_i = {
        let i = 10;
        let j = 20;
        let x = i + j;
        x
    };

    println!("{:?}", new_i);
}
