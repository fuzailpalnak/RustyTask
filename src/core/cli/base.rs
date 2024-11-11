use crate::manager::TaskManager;

use std::io::{self, Write};
pub trait EventCLI<T, M>
where
    M: TaskManager<T>,
{
    fn display_menu();
    fn process_input(&mut self, task_manager: &mut M);
}

pub fn welcome_message() -> String {
    println!("\nSelect an option:");
    println!("1: Manage Reminders");
    println!("2: Manage Events");
    println!("3: Exit");

    get_user_input()
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
