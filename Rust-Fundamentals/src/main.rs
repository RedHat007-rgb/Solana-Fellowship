fn main() {  
    println!("{}",is_even(5));  
    println!("The fib number is {}",fib(50));
    println!("the length of a string is {}", get_string_length("preetham reddy"));

    //Understanding and Implementing Structs 
    //Structs let us group the information an similar type together 

    struct User{
        username:String,
        password:String,
        age:u32
    }

    let user1=User{
        username:String::from("Reddy"),
        password:String::from("1234"),
        age:25,
    };

    println!("Name of the user: {}\nPassword is {} \nAge is {}",user1.username,user1.password,user1.age);
}

//Check the number if even and return  if even
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


// Take string as an input and return its length

fn get_string_length(input:&str)->i32{
    let mut temp=0;
    for _ in 0.. input.len(){ 
        temp=temp+1;
    }
    return temp;
    
}









