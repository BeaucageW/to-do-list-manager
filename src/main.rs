use std::{io::{self, Write}}; 
use serde::{Serialize, Deserialize};
fn main() {
    #[derive(Serialize, Deserialize)]
    struct Task{
        description: String,
        is_completed: bool,
    }
    impl Task{
        fn completion_symbol(&self) -> &str {
            if self.is_completed {
                return "✓"
            }else {
                return "X"
            }
        }  
    }   
    #[derive(Serialize, Deserialize)]
    struct TodoList {
        name: String,
        list: Vec<Task>,
    }
    impl TodoList {
        fn add(&mut self, description: String){
            self.list.push(Task { description: (description), is_completed: (false) });
        }

        fn print(&self){
            println!("{} : ", &self.name);
                for tasks in &self.list{
                    println!("- {}  [{}]", tasks.description, tasks.completion_symbol());
                }
        }

        fn complete(&mut self, index: usize){
            self.list[index].is_completed = true;
        }
        
        fn clear(&mut self){
            self.list = Vec::new();
        }

        fn save(&self) -> Result<(), Box<dyn std::error::Error>>{
            let self_as_string = serde_json::to_string(&self)?;
            std::fs::write("List.json", self_as_string)?;
            
            Ok(())
        }

        fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let self_as_string = std::fs::read_to_string("List.json")?;
            *self = serde_json::from_str(&self_as_string)?;
            
            Ok(())
        }
    }
    
    let mut todo_list: TodoList = TodoList { name: (String::from("My List")), list: (Vec::new()) };
    todo_list.load().expect("failed to load list");  
    
    fn get_input() -> String {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut  input)
            .expect("Invalid input !"); 
        
        input.trim().to_string()
    }

    fn detect_command(input: &str, todo_list: &mut TodoList) -> bool{
        let split_input: Vec<&str> = input.split_whitespace().collect();

        match split_input[0]{
            "/add" => {
                todo_list.add(split_input[1].to_owned());
                true
            }
            "/print" => {
                todo_list.print();
                true
            }
            "/quit" => false,
            
            "/complete" => {
                todo_list.complete(split_input[1].trim().parse::<usize>().expect("Unexpected argument"));
                true
            }
            "/help" => {
                println!("Available commands : 
                    '/complete <index>' completes the task at the selected index 
                    '/add <name>' creates a new task with the selected name 
                    '/print' prints the current list 
                    '/quit' exits the program 
                    '/clear' to clear the content of the list
                    '/save' to save the content of the list");
            true
            }
            "/clear" => {
                todo_list.clear();
                true
            }
            "/save" => {
                todo_list.save().expect("couldn't save data");
                true
            }
            other => {
                println!("command '{other}' not found. Type /help for list of commands");
                true
            }
        }  
    }

    loop{
        print!(">");
        io::stdout().flush().unwrap();

        if !detect_command(&get_input(), &mut todo_list){
            break;
        }      
    }
}