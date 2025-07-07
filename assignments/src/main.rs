use std::fs::{File, OpenOptions};
use std::io::prelude::*;


fn main() {
    println!("...........Reading a  Binary..........file.....");
     create_file();
    plus();
    plus();
    minus();
    delete_content();
}

fn create_file(){
    let mut file=File::create_new("a.bin").unwrap();
    file.write(b"50").unwrap();
}


fn plus(){
    let mut file=File::open("a.bin").unwrap();
    let mut buff=String::new();
    File::read_to_string(&mut file,&mut buff).unwrap();
    println!("{}",buff);
    let mut value:u8=buff.trim().parse().unwrap();
    value=value+1;
    
    println!("Incremented value: {}",value); 
}

fn minus(){
    let mut file=File::open("a.bin").unwrap();
    let mut buff=String::new();
    File::read_to_string(&mut file,&mut buff).unwrap();
    println!("{}",buff);
    let mut value:u32=buff.trim().parse().unwrap();
    value=value-1;
    println!("Decremented value: {}",value); 
}


fn delete_content(){
    OpenOptions::new().write(true).truncate(true).open("a.bin");
    println!("COntent CLeared....");
}







