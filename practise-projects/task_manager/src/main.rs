use std::io::stdin;
// static  mut tasks:Vec<String>=Vec::new();
fn main() {
    println!("*******.......Welcome to TASK_MANAGER.........**********");
    let mut tasks:Vec<String>=Vec::new();
     loop{
        menu();
        let mut input:String=String::new();
        println!("PLease enter a number to do something...");
        stdin().read_line(&mut input).expect("please enter the valid value");
        let mutated_value=input.trim();
        if(mutated_value == "1"){
            let result=add_task(&mut tasks);
            println!("☺️Successfully added Task:{result}")
            
        }else if mutated_value== "2" {
            remove_task();  
        }
        else if mutated_value=="3"{
            edit_task();
        } 
        else if mutated_value=="4"{
            view_tasks(&mut tasks);  
        } else if mutated_value =="5"{
            break;
        }
    }
}


fn menu(){
    
    println!("1 . Add a Task.");
    println!("2 . Remove a Task.");
    println!("3 . Edit a Task.");
    println!("4 . View all Task.");
    println!("5 .Exit");
}

fn add_task(tasks:&mut Vec<String>)->String{
    let mut new_task=String::new();
    println!("please  give a name to a task: ");
    stdin().read_line(&mut new_task).expect("adding unsuccessfull");
    let mut task=&mut new_task;
    
    tasks.push(task.to_string());
    return task.to_string();
    
   
}
fn remove_task(){
    println!("in add tasks");
    
}
fn edit_task(){
    println!("in add tasks");
    
}
fn view_tasks(tasks:&mut Vec<String>){

    println!("IN view tasks");
  
}
// fn user_input()->String{
        
    
//     let value=input.trim().to_string();
//      return value;
// }