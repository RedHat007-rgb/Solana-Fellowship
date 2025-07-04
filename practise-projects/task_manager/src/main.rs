use std::io::stdin;
// static  mut tasks:Vec<String>=Vec::new();
fn main() {
    
    let mut tasks:Vec<String>=Vec::new();
     loop{
        println!("*******.......Welcome to TASK_MANAGER.........**********");
        menu();
        let mut input:String=String::new();
        println!("👺😡Please enter a number to do something???????...");
        stdin().read_line(&mut input).expect("please enter the valid value");
        let mutated_value=input.trim();
        if(mutated_value == "1"){
            let result=add_task(&mut tasks);
            println!("☺️......Successfully added Task:{result}....")
            
        }else if mutated_value== "2" {
            remove_task(&mut tasks);  
        }
        else if mutated_value=="3"{
            edit_task();
        } 
        else if mutated_value=="4"{
            println!("Tasks: " );
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
    let task= new_task.trim(); 
    tasks.push(task.to_string());
    return task.to_string()
    
   
}
fn remove_task(tasks:&mut Vec<String>){
    let mut task=String::new();
    
    println!("please enter the task name to delete: ");
    stdin().read_line(&mut task).expect("error reading line");
    let delete_task=task.trim().to_lowercase();
    if delete_task.len()==0 {
        println!("🐒 please dont leave empty.please enter a task:")
    }
    let mut found=false;
    for i in 0..tasks.len(){
        print!("in for loop");
        println!("delete: {}",delete_task);
        println!("tasks: {}",tasks[i].to_lowercase());
        if tasks[i].to_lowercase().trim()== delete_task.trim(){
             tasks.remove(i);
             println!("in if loop");
              found=true;
              break;
        }
        
        
    }
    println!("{}",found);
    if found==true {
        println!("{} ...Successfully deleted....", delete_task);
    } else {
        println!("🥹 Task not found: '{}'", delete_task);
    }    
}
fn edit_task(){
    println!("in add tasks");
    
}


fn view_tasks(tasks:&mut Vec<String>){
    
    if tasks.len()==0{
        println!("😉 No Tasks available.Please try after adding tasks...")
      
    }else{
        for i in  0..tasks.len(){
        
        println!("{} .{}",i+1,tasks[i]);
      }
    }
    
}
