// use std::any::{type_name, type_name_of_val};

fn main() {
    let x=5;
    println!("The value of x is:{x}");
    /*The below code doesn't compile because by default  variables are immutable
     * We can still make them mutable by using miut keyword..
     * So rust give advantage of  type safety  and mutability..
     */

    // x=5;
    // println!("The value of x is:{x}");


    let mut y=10;
    println!("The value of y is: {y}");
    y=90;
    println!("The value of y is :{y}");
    /*
        The above value 10 is assighned to variable y in line 13
        and in the next line it prints 10 but 
        In the same line we can observe that mut keyword is used 
        before varibable y(i.e.. the value of y might be changed)
        This helps the future readers that this variable value might be chhanged 
        in the below code
     */
    println!("..................************.....................");

    const  RANDOM_NUMBER:u32=30*78;
    println!("{RANDOM_NUMBER}");
    // for i in 1..100{
    //     const TOTAL_SUM:u64=i+1;

    // }
    /*
    CONSTANTS:
        constants by the name itself they cant be changed as same as 
        variables in rust by default but there is a slight  difference between them
        -->need to be named in UPPERCASE LETTERS with underscores.
        -->They can be declared within or global scope for our convinient.
        -->Once assigned u can the change the value of a constant.
        -->const cant be assigned  that needs to computation at runtime...
     */
    println!("..................************.....................");
    let a=40;
    let a=a+a+20;
    {
        let a=40*32*a;
        println!("in scope:{a}");
    }
    println!("global value of a: {a}");

    let spaces="  ";
    
    let spaces=spaces.len();
    println!("Spaces: {}",spaces);

    /*
        here the type of line 53  is string with variable name space;
        but ion the next line as we are finding the length so the types has beed 
        changed to usize;

        but if we do with the concept of mutability,it will be a compilation error..

     */

     println!("..................************.....................");
    let p:u8=255;
    let c=p.wrapping_add(34);
    println!("INTEGER OVERFLOW:{}",c);

    println!("..................************.....................");

    /*
    Compund Data Types:
        Tuples and arrays:

        Tuples:tuples are nothing but they can group all  values with diff data types)
        but one condition once declared the size cant be changed..
        in my understanding tuples  is a box which can hold diff types
     */

    let person=("bala",8.9,90);
    let (a,b,c)=person;
    println!("{}",person.2);
    println!("{}",a);



   
    


    /*
        Shadowing:
            In the first line of code we assigned 40 ti variable a;
            In teh secont statement;We declare the same  variabllw with let keyword with a+a+20;
            i.e..,the second line a is overshadowinmg the first line a but still it has the value of a when it computes teh second line.

            -->as you see in the scope again new variable with the same name still it as a value whuch is the previous one;
            But in the scope it gets compiled and it will be in that scope

            -->In the next line when user prints a which is global it prints some big value because still the first a is overshadowed by 2nd.
            
     */
    println!("..................************.....................");

    
    println!("..................************.....................");
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
    }

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
    }

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