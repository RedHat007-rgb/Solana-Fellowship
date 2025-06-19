use std::io;

fn main() {
    println!("Welcome to Basic Rust Calculator");
    println!("Please Enter the Operation (add/sub/mul/div):");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();
    let operation = operation.trim();

    println!("Please Enter the number 1:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let trimmed1 = input1.trim();
    let num1: f32 = trimmed1.parse().unwrap();

    println!("Please Enter the number 2:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let trimmed2 = input2.trim();
    let num2: f32 = trimmed2.parse().unwrap();

    
    let mut ans = 0.0;

    if operation == "add" {
        ans = add(num1, num2);
        println!("Answer: {}", ans);
    } else if operation == "sub" {
        ans = subtract(num1, num2);
        println!("Answer: {}", ans);
    } else if operation == "mul" {
        ans = multiplication(num1, num2);
        println!("Answer: {}", ans);
    } else if operation == "div" {
        if num2 == 0.0 {
            println!("Cannot divide by zero.");
        } else {
            ans = division(num1, num2);
            println!("Answer: {}", ans);
        }
    } else {
        println!("Invalid operation.");
    }

    let sol_value = ans / 100.0;
    println!("Equivalent in SOL: {} SOL", sol_value);
}

fn add(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

fn subtract(num1: f32, num2: f32) -> f32 {
    num1 - num2
}

fn multiplication(num1: f32, num2: f32) -> f32 {
    num1 * num2
}

fn division(num1: f32, num2: f32) -> f32 {
    num1 / num2
}
