use super::base::{EventCLI, CLI};
use chrono::NaiveDateTime;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::manager::base::TaskManager;
use crate::manager::reminder_manager::ReminderTaskManager;
use crate::tasks::base::Priority;
use crate::tasks::reminder::Reminder;
pub struct ReminderCLI;

impl EventCLI<ReminderTaskManager<Reminder>> for ReminderCLI {
    fn display_menu() {
        println!("\nReminder Management:");
        println!("1: Add Task");
        println!("2: View Tasks");
        println!("3: Delete Task");
        println!("4: Exit");
    }

    async fn process_input(&mut self, task_manager: &Arc<RwLock<ReminderTaskManager<Reminder>>>) {
        Self::display_menu();
        let option = CLI::get_user_input();

        match option.as_str() {
            "1" => match Self::get_reminder() {
                Ok(reminder) => {
                    let mut task_manager = task_manager.write().await;
                    task_manager.add(reminder);
                }
                Err(_) => println!("Failed to add reminder."),
            },
            "2" => {
                let task_manager = task_manager.read().await;
                task_manager.view();
            }
            "3" => match Self::get_id_from_user_promt() {
                Ok(id) => {
                    let mut task_manager = task_manager.write().await;
                    task_manager.delete(id);
                }
                Err(_) => println!("Failed to delete reminder."),
            },
            _ => println!("Invalid option. Please try again."),
        }
    }
}

impl ReminderCLI {
    pub fn get_reminder() -> Result<Reminder, String> {
        println!("Enter task title:");
        let title = CLI::get_user_input();
        println!("Enter task description:");
        let description = CLI::get_user_input();
        println!("Enter priority (Low, Medium, High):");
        let priority = match CLI::get_user_input().to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => return Err("Invalid priority. Please enter Low, Medium, or High.".to_string()),
        };
        println!("Enter the due date and time (YYYY-MM-DD HH:MM):");
        let due_datetime_input = CLI::get_user_input();
        let due_datetime =
            match NaiveDateTime::parse_from_str(&due_datetime_input, "%Y-%m-%d %H:%M") {
                Ok(datetime) => datetime,
                Err(_) => {
                    return Err("Invalid date-time format. Please use YYYY-MM-DD HH:MM.".to_string())
                }
            };
        let reminder = Reminder::new(title, description, due_datetime, priority);
        Ok(reminder)
    }

    pub fn get_id_from_user_promt() -> Result<i32, ()> {
        println!("Enter task ID to delete:");
        CLI::get_user_input().parse().map_err(|_| ())
    }
}
