mod todo;
mod utils;

use std::path::Path;

use std::fs::{File,read_to_string};

use std::io::{self, 
        Write};

use utils::{get_title, 
    get_description, 
    get_due_date, 
    get_priority, 
    get_status};

use todo::{Todo, 
        Status};

use clap::{Parser, 
        Subcommand};

#[derive(Parser)]
#[command(name = "Todo-app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
}

#[derive(Subcommand)]
enum Commands {
    Run,
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Run) => {
            run();
        }
        None => {
            println!("No command provided. Use 'run' to start the application."); 
        }
    }
}

fn run() {
    let path = Path::new("files/todo.json");
    let mut list: Vec<todo::Todo>;
    if path.exists() {
        let data = read_to_string(path).unwrap();
        list = serde_json::from_str(&data).unwrap();
        println!("Welcome to ToDo_app...");
        println!("Loaded {} tasks from file.", list.len());
    } else {
        list = Vec::new();
        println!("Welcome to ToDo_app...");
    }
    
    loop {
        print!("$ ");
        io::stdout()
            .flush()
            .expect("Failed to read line");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();

        match input {
            "add" => {
                let title = get_title();
                let description = get_description();
                let due_date = get_due_date();
                let priority = get_priority();
                let status = get_status();

                let todo = Todo::new(
                    title,
                    description,
                    due_date,
                    priority,
                    status,
                );
                list.push(todo);
            }
            "display" => {
                if list.is_empty() {
                    println!("No tasks at the moment.");
                   
                }else {
                    for todo in &list {
                        todo.display();
                        println!("---------------------------------");
                    }
                }
                
            }
            "change-priority" => {
                println!("Enter the Task ID to update priority:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                if let Ok(id) = id_input.trim().parse::<i32>() {
                    if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
                        let new_priority = get_priority();
                        todo.set_priority(new_priority);
                        println!("Priority updated.");
                    } else {
                        println!("Todo with ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "change-status" => {
                println!("Enter the Task ID to update status:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                if let Ok(id) = id_input.trim().parse::<i32>() {
                    if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
                        let new_status = get_status();
                        match new_status {
                            Status::Completed => todo.mark_completed(),
                            Status::Incompleted => todo.mark_incompleted(),
                            Status::Inprogress => todo.mark_inprogress(),
                        }
                        println!("Status updated.");
                    } else {
                        println!("Todo with ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "change-description" => {
                println!("Enter the Task ID to update description:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                if let Ok(id) = id_input.trim().parse::<i32>() {
                    if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
                        let new_description = get_description();
                        todo.set_description(new_description);
                        println!("Description updated.");
                    } else {
                        println!("Todo with ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "change-due-date" => {
                println!("Enter the Task ID to update due date:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                if let Ok(id) = id_input.trim().parse::<i32>() {
                    if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
                        let new_due_date = get_due_date();
                        todo.set_due_date(new_due_date);
                        println!("Due date updated.");
                    } else {
                        println!("Todo with ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "delete" => {
                println!("Enter the Task ID to delete:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                if let Ok(id) = id_input.trim().parse::<i32>() {
                    if let Some(pos) = list.iter().position(|t| t.id == id) {
                        list.remove(pos);
                        println!("Todo with ID {} deleted.", id);
                    } else {
                        println!("Todo with ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "save" => {
                let json = serde_json::to_string_pretty(&list).unwrap();
                let mut file = File::create("files/todo.json").unwrap();
                file.write_all(json.as_bytes()).expect("Failed to write to file");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            "--help" => {
                println!("Available commands:");
                println!("add - Add a new task");
                println!("change-priority - Set the priority of a task");
                println!("change-status - Set the status of a task");
                println!("change-description - Set the description of a task");
                println!("change-due-date - Set the due date of a task");
                println!("delete - Delete a task");
                println!("display - Display all task");
                println!("save - Save tasks to file");
                println!("exit - Exit the application");
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
    
}

