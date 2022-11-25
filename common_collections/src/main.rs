fn main() {
    // to create a vector in rust we use Vec::new
    // then we add the values with `.push`

    // let mut v: Vec<i32> = Vec::new();
    // for i in 0..10 {
    //     v.push(i * 5);
    // }

    // same as new_array = [0] * 10

    // let mut new_vector = vec![0; 10];

    // for i in v.iter().enumerate() {
    //     println!("{:?} => {:?} ", i.0, i.1);
    // }

    // println!("{:?}", new_vector);

    // imagine the vectors like a List
    // index do exists in the vector

    // new_vector[0] = 1;
    // println!("{:?}", new_vector);

    let new_vector_third = vec![1, 2, 3, 4, 5];

    // let four = &new_vector_third[3];

    for i in &new_vector_third {
        println!("{i}");
    }

    // println!("{}", four);
}
