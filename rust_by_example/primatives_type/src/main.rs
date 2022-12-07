fn reverse_num(pair: (i32, i32)) -> (i32, i32) {
    let (one, two) = pair;
    (two, one)
}

fn main() {
    // tuples are data structure which can store different datatypes (string, int, char, bool etc)
    let new_tuples = (1, 'A', true, "Arsh");
    // getting the value by the index.
    println!("{}", new_tuples.0);
    println!("{:?}", reverse_num((1, 2)));

    // arrays and slice
    // elements of same type
    // decalation [T; length]
    let new_array: [i32; 5] = [1, 2, 3, 4, 5];
    // to declare dummy array
    let mut dummy_array = [0; 5];
    // it will create an array of [0, 0, 0, 0, 0]
    for (idx, val) in new_array.iter().enumerate() {
        println!("{idx} -- {val}");
        dummy_array[idx] = *val;
    }

    println!("{:?}", dummy_array);
}
