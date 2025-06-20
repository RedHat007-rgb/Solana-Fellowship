use std::io::stdin;

use rand::Rng;

fn main() {
    println!("Gueessing game");
    println!("Enter the number to guess:");
    let mut number=String::new();
    stdin().read_line(&mut number).expect("crash here");
    println!("you entered: {number}");

    let random_number=rand::rng().random_range(1..100);

    println!("Secret number is {random_number}")

   
}
