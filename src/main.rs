mod todo;
mod utils;
mod commands;



use std::path::Path;

use std::fs::{read_to_string};

use std::io::{self, 
        Write};



use clap::{Parser, 
        Subcommand};
use crate::commands::{add, change_description, change_duedate, change_priority, change_status, delete, display, exit, help, save};

#[derive(Parser)]
#[command(name = "todo-cli",
    about = "A simple command-line todo list application",
    long_about = "Todo-cli is a lightweight tool for managing your tasks from the terminal. \
                  Use it to run the application and manage your todos efficiently.",
    version = "0.1.0",
    author = "Abhijna S P <abhijnasp1@gmail.com>")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Run the todo application",
        long_about = "Starts the todo application, allowing you to interact with your task list."
    )]
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
    let path = Path::new("../files/todo.json");
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
                add(&mut list);
            }
            "display" => {
                display(&mut list);
            }
            "change-priority" => {
                change_priority(& mut list);
            }
            "change-status" => {
                change_status(& mut list);
            }
            "change-description" => {
                change_description(& mut list);
            }
            "change-due-date" => {
                change_duedate(& mut list);
            }
            "delete" => {
                delete(&mut list);
            }
            "save" => {
               save(& mut list);
            }
            "exit" => {
                exit();
                break;
            }
            "--help" => {
                help()
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
    
}

