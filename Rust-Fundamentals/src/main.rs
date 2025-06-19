
//Check the number if even and return  if even
fn main() {  
    println!("{}",is_even(5));  
    println!("The fib number is {}",fib(50));
}
fn is_even(num:i32)-> bool{
    if num%2==0 {
       return true;
    }else{
        return false;
    }
}

//Fibonacci number

fn fib(num:u64)->u64{
    let mut first_number=0;
    let mut second_number=1;
    if num ==0{
        return first_number;
    }
    if num ==1{
        return second_number;
    }

    for _ in 0..(num-2){
        let temp = second_number;
        second_number=first_number+second_number;
        first_number=temp;
    }
    return second_number;
}