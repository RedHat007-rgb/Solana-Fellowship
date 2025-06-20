use std::io::stdin;

fn main() {
    println!("Gueessing game");
    println!("Enter the number to guess:");
    let mut number=String::new();
    stdin().read_line(&mut number).expect("crash here");
    println!("you entered: {number}")
}
