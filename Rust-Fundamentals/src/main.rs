use std::fmt::Display;
fn main() {
    // let y=100;
    // let final_result=add(y);
    // println!("{final_result}");
    // let p=y;
    // println!("{p}");


    // let a=String::from("hello....");
    // // let exp_str_return=exp_str(a);
    // let exp_str_return=exp_str_modify(a);
    // println!("value returned: {exp_str_return}");
    // // let x:String=a; This is wrong......
    
    let name=String::from("hello");
    let return_value=tuple(name);
    println!("{:?} ",return_value,);



}

fn add(x:i32)->i32{
    let result=x+100;
    return result;
}

fn exp_str(s:String)->String{
    return s;
}

fn exp_str_modify(s:String)->String{
    let result=s+"world.....";
    return result;
}
  
fn tuple(s:String)->(String,String){
    let string=s.clone();
    let modified_string=String::from(string+"hello..");
    return (s,modified_string);
}

