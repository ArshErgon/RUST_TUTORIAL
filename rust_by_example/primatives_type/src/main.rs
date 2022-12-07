fn reverse_num(pair: (i32, i32)) -> (i32, i32) {
    let (one, two) = pair;
    (two, one)
}

fn get_sliced_size(pair: &[i32]) -> (u32, bool) {
    let is_empty = pair.len() == 0;
    let length: u32 = pair.len().try_into().unwrap();
    return (length, is_empty);
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
    // * for taking the value for later use
    for (idx, val) in new_array.iter().enumerate() {
        println!("{idx} -- {val}");
        dummy_array[idx] = *val;
    }

    println!("{:?}", dummy_array);

    // slicing in array

    let sliced_array: [i32; 5] = [1, 2, 3, 4, 5];
    let (x, y) = get_sliced_size(&sliced_array[0..3]);
    println!("{x} -- {y}");

    for i in 0..sliced_array.len() + 1 {
        match sliced_array.get(i) {
            Some(sval) => println!("{i} {sval}"),
            None => println!("Slow down to Index: {i}"),
        }
    }
}
