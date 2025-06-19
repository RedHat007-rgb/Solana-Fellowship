
//Check the number if even and return  if even
fn main() {  
    println!("{}",is_even(5));  
}
fn is_even(num:i32)-> bool{
    if num%2==0 {
       return true;
    }else{
        return false;
    }
}
