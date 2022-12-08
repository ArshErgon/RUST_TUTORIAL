fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    // `loop` keyword is used for infinite looping

    let mut idx = 0;
    loop {
        if idx == 5 && idx < 6 {
            println!("Hit {idx}");
        }

        println!("{idx}");

        if idx > 6 {
            println!("that would be enough\nLast Index hit {idx}");
            break;
        }

        idx += 1;
    }
    println!("Loop break");

    // nested loop

    let mut inner_count = 0;
    let mut most_inner_count = 0;

    'outer: loop {
        println!("Loop start");
        'inner: loop {
            inner_count += 5;
            'most_inner: loop {
                if most_inner_count == inner_count {
                    println!("breaking inner loop here");
                    break 'inner;
                }
                most_inner_count += 1;
            }
        }
        println!("outer break");
        break 'outer;
    }

    let mut counter = 0;

    let counter_value = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{counter_value}");

    // while loop

    let mut i = 1;

    while i <= 3 {
        if i == 1 {
            println!("One");
        } else if i == 2 {
            println!("Two")
        } else {
            println!("Three");
        }
        i += 1;
    }

    // for loop in RUST

    // 0..5 will print till 4
    // use = to print full range
    // i.e: 0..=5

    for i in 0..=5 {
        println!("{i}");
    }

    // looping into arrays, or vectors iterators

    let names = vec!["arsh", "ergon", "ali"];

    for n in names.into_iter() {
        match n {
            "arsh" => println!("owner"),
            _ => println!("not allowed"),
        }
    }

    // match similar like C's switch

    let num = 10;
    match num {
        1 => println!("number is 1"),
        1..=5 => println!("in between of 1 to 5"),
        6 | 8 | 10 => println!("number comes in different section {num}"),
        _ => println!("invalid"),
    }

    // using guards in match; simply using if-else in the match case

    match num {
        num if num > 5 => println!("num is bigger than 5"),
        num if num == 10 => println!("num is {num}"),
        _ => println!("invalid"),
    }

    // the use of if-let
    
}
