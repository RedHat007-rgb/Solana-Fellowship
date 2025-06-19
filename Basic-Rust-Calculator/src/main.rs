use std::io;

fn main(){
    println!("Welcome to Basic Rust Calculator");
    println!("Please Enter the Operation add/sub/mul/div)");
    let mut operation=String::new();
    io::stdin().read_line(&mut operation).unwrap();

    println!("Please Enter the number 1");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let trimmed =input1.trim();
    let  num1:f32=trimmed.parse().unwrap();

    println!("Please Enter the number 2");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let trimmed =input2.trim();
    let  num2:f32=trimmed.parse().unwrap();
    
   
}

fn add(num1:f32,num2:f32)->f32{
   return num1+num2;
}

fn substract(num1:f32,num2:f32)->f32{
    return num1-num2;
}

fn multiplication(num1:f32,num2:f32)->f32{
    return num1*num2;
}
fn division(num1:f32,num2:f32)->f32{
   return num1/num2;
    
}
