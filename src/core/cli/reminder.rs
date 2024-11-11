use crate::cli::{get_user_input, EventCLI};
use crate::manager::{ReminderTaskManager, TaskManager};
use crate::tasks::{Priority, Reminder};
use chrono::NaiveDateTime;

pub struct ReminderCLI;

impl EventCLI<Reminder, ReminderTaskManager<Reminder>> for ReminderCLI {
    fn display_menu() {
        println!("\nReminder Management:");
        println!("1: Add Task");
        println!("2: Mark Task Complete");
        println!("3: Update Task Priority");
        println!("4: View Tasks");
        println!("5: Delete Task");
        println!("6: Exit");
    }

    fn process_input(&mut self, task_manager: &mut ReminderTaskManager<Reminder>) {
        Self::display_menu();
        let option = get_user_input();

        match option.as_str() {
            "1" => match Self::get_reminder() {
                Ok(reminder) => task_manager.add_task(reminder),
                Err(_) => println!("Failed to add reminder."),
            },
            "5" => match Self::delete_task() {
                Ok(id) => task_manager.delete_task(id),
                Err(_) => println!("Failed to delete reminder."),
            },
            _ => println!("Invalid option. Please try again."),
        }
    }
}

impl ReminderCLI {
    pub fn get_reminder() -> Result<Reminder, ()> {
        println!("Enter task title:");
        let title = get_user_input();
        println!("Enter task description:");
        let description = get_user_input();
        println!("Enter priority (Low, Medium, High):");
        let priority = match get_user_input().to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => Priority::Low,
        };
        println!("Enter the due date and time (YYYY-MM-DD HH:MM):");
        let due_datetime = NaiveDateTime::parse_from_str(&get_user_input(), "%Y-%m-%d %H:%M")
            .expect("Invalid date-time format");

        Ok(Reminder::new(title, description, due_datetime, priority))
    }

    pub fn delete_task() -> Result<i32, ()> {
        println!("Enter task ID to delete:");
        get_user_input().parse().map_err(|_| ())
    }
}
