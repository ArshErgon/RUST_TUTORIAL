// Stuck are same like we have in C or C++

// struct USER {
//     name: String,
//     age: u8,
// }

// Struck by tuples
// struct Tuples (u32, String, i64);

// a basic struct program
// plus we have used this for method

struct Area {
    width: u32,
    height: u32,
}

// self like we use for python
// think struct as a class and `impl` as function

impl Area {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_it_valid(&self) -> bool {
        if self.width <= 0 || self.height <= 0 {
            false
        } else {
            true
        }
    }
}

fn main() {
    // using struct for creating a new_user by the help of function

    // let name = create_user("Arsh".to_string(), 19);
    // println!("{0} ;; {1}", name.name, name.age);

    // let color_name = Tuples(32, "Arsh".to_string(), 2*999);
    // println!("{0}", color_name.1);

    // A program for basic struct program

    // let area = Area {
    //     width: 2,
    //     height: 5,
    // };

    // println!("{}", area_fun(area.width, area.height));

    // Methods using the struct

    let react_one = Area {
        width: 10,
        height: 10,
    };

    let check: bool = react_one.is_it_valid();
    if check {
        println!("{}", react_one.area());
    }
}

// function that return area

// fn area_fun(width: u32, height: u32) -> u32 {
//     width * height
// }

// creating a new_user with the help of function
// remember if you are using this struct template
// so use `struct name` as a return type for the function.

// fn create_user(name: String, age: u8) -> USER {
//     USER { name, age }
// }
