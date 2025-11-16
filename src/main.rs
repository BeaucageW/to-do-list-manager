use std::{io::{self, Write}}; 

fn main() {
    struct ToDo{
        description: String,
        is_completed: bool,
    }    
    
    let mut todo_list: Vec<ToDo> = Vec::new();

    fn complete_command(todo: &mut ToDo){
        todo.is_completed = true;
    }
    
    fn print_command(list: &Vec<ToDo>){
        println!("To Do List : ");
                for todos in list{
                    println!("- {}  [{}]", todos.description, completion_symbol(&todos));
                }
    }

    fn add_command(description: &str, todo_list: &mut Vec<ToDo>){
        todo_list.push(ToDo { description: (description.to_string()), is_completed: (false) });
    }
    
    fn completion_symbol(todo: &ToDo) -> &str {
        if todo.is_completed {
            return "✓"
        }else {
            return "X"
        }
    }  
    
    fn get_input() -> String {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut  input)
            .expect("Invalid input !"); 
        
        input.trim().to_string()
    }
    
    fn detect_command(input: &str, todo_list: &mut Vec<ToDo>) -> bool{
        let split_input: Vec<&str> = input.split_whitespace().collect();

        match split_input[0]{
            "/add" => {
                add_command(split_input[1], todo_list);
                true
            }
            "/print" => {
                print_command(todo_list);
                true
            }
            "/quit" => false,
            
            "/complete" => {
                complete_command(&mut todo_list[split_input[1].trim().parse::<usize>().expect("Incorrect parameter found for command 'complete'")]);
                true
            }
            "/help" => {
                println!("Available commands : 
                    '/complete <index>' completes the task at the selected index 
                    '/add <name>' creates a new task with the selected name 
                    '/print' prints the current list 
                    '/quit' exits the program ");
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