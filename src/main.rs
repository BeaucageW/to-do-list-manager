use std::io::{self, Write}; 

fn main() {
    struct ToDo{
        description: String,
        _is_completed: bool, // will be implemented later 
    }    
    
    let mut todo_list: Vec<ToDo> = Vec::new();

    fn get_input() -> String {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut  input)
            .expect("Invalid input !"); 
        
        input.trim().to_string()
    }

    'program: loop{
        print!(">");
        io::stdout().flush().unwrap();

        match get_input().as_str(){
            "print" =>{ 
                println!("To Do List : ");
                for todo in &todo_list{
                    println!("- {}", todo.description);
                }
            }
            
            "quit" | "exit" => break 'program,
            
            other => todo_list.push(ToDo { description: (other.to_string()), _is_completed: (false) }),
        }
    }
}