fn main() {
    println!("{}", is_even(5));
    println!("..................************.....................");
    println!("The fib number is {}", fib(50));
    println!("..................************.....................");
    println!(
        "the length of a string is {}",
        get_string_length("preetham reddy")
    );
    println!("..................************.....................");

    //Understanding and Implementing Structs
    //Structs let us group the information an similar type together
    //Structs are more closer to java as we can implementing functions in the struct.

    struct User {
        username: String,
        password: String,
        age: u32,
    }

    let user1 = User {
        username: String::from("Reddy"),
        password: String::from("1234"),
        age: 25,
    };

    struct Square {
        side: u32,
    };

    impl Square {
        fn area(&self) -> u32 {
            return self.side * self.side;
        }

        fn perimeter(&self) -> u32 {
            return 4 * self.side;
        }

        fn print() -> String {
            let new_string = String::from("You have a square in your hand");
            return new_string;
        }
    };

    let sq1 = Square { side: 90 };

    println!("Area of sq1 is{}", sq1.area());
    println!("Perimeter of sq1 is {}", sq1.perimeter());

    println!("{}", Square::print());

    println!("..................************.....................");
    println!(
        "Name of the user: {}\nPassword is {} \nAge is {}",
        user1.username, user1.password, user1.age
    );
    println!("..................************.....................");
}

//Check the number if even and return  if even
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

//Fibonacci number

fn fib(num: u64) -> u64 {
    let mut first_number = 0;
    let mut second_number = 1;
    if num == 0 {
        return first_number;
    }
    if num == 1 {
        return second_number;
    }

    for _ in 0..(num - 2) {
        let temp = second_number;
        second_number = first_number + second_number;
        first_number = temp;
    }
    return second_number;
}

// Take string as an input and return its length

fn get_string_length(input: &str) -> i32 {
    let mut temp = 0;
    for _ in 0..input.len() {
        temp = temp + 1;
    }
    return temp;
}
