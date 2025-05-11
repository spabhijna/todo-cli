use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

static INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Todo {
    pub id:i32,
    title: String,
    description: String,
    due_date: String,
    priority: Priority,
    status: Status,  
}

#[derive(Serialize,Deserialize,Debug,Clone,Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
}
impl Priority {
    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_string(),
            Priority::Medium => "Medium".to_string(),
            Priority::High => "High".to_string(),
        }
    }
}
#[derive(Serialize,Deserialize,Debug,Clone,Copy)]
pub enum Status {
    Completed,
    Incompleted,
    Inprogress,

}
impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Completed => "Completed".to_string(),
            Status::Incompleted => "Incompleted".to_string(),
            Status::Inprogress => "Inprogress".to_string(),
        }
    }
}


impl Todo {
    pub fn new(title: String, description: String, due_date: String, priority: Priority, status: Status) -> Self {
        let todo=Todo {
            id: INSTANCE_COUNT.load(Ordering::SeqCst) as i32,
            title,
            description,
            due_date,
            priority,
            status,
        };
        INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
        return todo;
        
         
    }
    pub fn display(&self) {
        println!("Todo ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Due Date: {}", self.due_date);
        println!("Priority: {}", self.priority.to_string());
        println!("Status: {}", self.status.to_string());
    }
    pub fn mark_completed(&mut self) {
        self.status = Status::Completed;
    }
    pub fn mark_incompleted(&mut self) {
        self.status = Status::Incompleted;
    }
    pub fn mark_inprogress(&mut self) {
        self.status = Status::Inprogress;
    }
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
    pub fn set_due_date(&mut self, due_date: String) {
        self.due_date = due_date;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}