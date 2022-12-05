use std::collections::HashMap;

fn main() {
    // syntax:
    // map.insert(key, value)

    let mut hash_map = HashMap::new();
    hash_map.insert(String::from("Arsh"), 20);

    // for (key, value) in hash_map {
    //     println!("{key} -- {value}");
    // }

    hash_map.entry(String::from("Ergon")).or_insert(50);
    hash_map.entry(String::from("Arsh")).or_insert(50);

    println!("{}", hash_map["Ergon"]);
    println!("{}", hash_map["Arsh"]);

    // putting the text into the hashMap

    let new_string = "Arsh Ergon Open Source Developer Arsh".to_string();

    let mut new_string_map = HashMap::new();

    // count used for increment the value

    for word in new_string.split_whitespace() {
        let count = new_string_map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &new_string_map {
        println!("{key} -- {value}");
    }

    println!("{}", new_string_map.len());
}
