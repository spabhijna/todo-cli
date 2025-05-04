use std::io;
use crate::todo::{Priority, Status};

pub fn get_title() -> String {
    println!("Enter the title of the todo item item");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let title = title.trim().to_string();
    title
}

pub fn get_description() -> String {
    println!("Enter the description of the todo item");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let description = description.trim().to_string();
    description
}

pub fn get_priority() -> Priority {
    println!("Enter priority (1: Low, 2: Medium, 3: High): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "1" => Priority::Low,
        "2" => Priority::Medium,
        "3" => Priority::High,
        _ => {
            println!("Invalid input, defaulting to Low.");
            Priority::Low
        }
    }
}

pub fn get_status() -> Status {
    println!("Enter status (1: Completed, 2: Incompleted, 3: Inprogress): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "1" => Status::Completed,
        "2" => Status::Incompleted,
        "3" => Status::Inprogress,
        _ => {
            println!("Invalid input, defaulting to Incompleted.");
            Status::Incompleted
        }
    }
}

pub fn get_due_date() -> String {
    println!("Enter the due date (YYYY-MM-DD): ");
    let mut due_date = String::new();
    io::stdin()
        .read_line(&mut due_date)
        .expect("Failed to read line");
    let due_date = due_date.trim().to_string();
    due_date
}