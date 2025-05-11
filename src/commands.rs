use std::fs::File;
use std::io;
use crate::todo::{Status, Todo};
use crate::utils::{get_description, get_due_date, get_priority, get_status, get_title};
use std::io::prelude::{Write};

pub fn add(list:&mut Vec<Todo>)  {
    {
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
}

pub fn display(list:& mut Vec<Todo>) {
    if list.is_empty() {
        println!("No tasks at the moment.");

    }else {
        for todo in list {
            todo.display();
            println!("---------------------------------");
        }
    }
}

pub fn change_priority(list:& mut Vec<Todo>) {
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

pub fn change_status(list:& mut Vec<Todo>) {
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

pub fn change_description(list:& mut Vec<Todo>) {
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

pub fn change_duedate(list:& mut Vec<Todo>) {
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

pub fn delete(list:& mut Vec<Todo>) {
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
pub fn save(list:& mut Vec<Todo>) {
    let json = serde_json::to_string_pretty(list).unwrap();
    let mut file = File::create("../files/todo.json").expect("Failed to create json file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
    println!("Content saved.");
}
pub fn exit() {
    println!("Exiting...");
    
}
pub fn help() {
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
