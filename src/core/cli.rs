use crate::manager::{ReminderTaskManager, TaskManager};
use crate::tasks::{Priority, Reminder};
use chrono::NaiveDateTime;

pub enum EventTypeCLI {
    Ok(ReminderCLI),
}

pub trait EventCLI<M> {
    fn display_menu();
    fn process_input(&mut self, task_manager: &mut M);
}

pub struct CLI {}
pub struct ReminderCLI;

impl CLI {
    pub fn welcome_message() {
        println!("\nSelect an option:");
        println!("1: Manage Reminders");
        println!("2: Manage Events");
        println!("3: Exit");
    }

    pub fn get_user_input() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    pub fn get_event_cli() -> Option<EventTypeCLI> {
        Self::welcome_message();
        let option = Self::get_user_input();

        match option.as_str() {
            "1" => Some(EventTypeCLI::Ok(ReminderCLI)),
            "3" => None,
            _ => None,
        }
    }
}

impl EventCLI<ReminderTaskManager<Reminder>> for ReminderCLI {
    fn display_menu() {
        println!("\nReminder Management:");
        println!("1: Add Task");
        println!("2: Mark Task Complete");
        println!("3: View Tasks");
        println!("4: Delete Task");
        println!("5: Exit");
    }

    fn process_input(&mut self, task_manager: &mut ReminderTaskManager<Reminder>) {
        Self::display_menu();
        let option = CLI::get_user_input();

        match option.as_str() {
            "1" => match Self::get_reminder() {
                Ok(reminder) => task_manager.add(reminder),
                Err(_) => println!("Failed to add reminder."),
            },
            "2" => match Self::get_id_from_user_promt() {
                Ok(id) => task_manager.complete(id),
                Err(_) => println!("Failed to delete reminder."),
            },
            "3" => task_manager.view(),
            "4" => match Self::get_id_from_user_promt() {
                Ok(id) => task_manager.delete(id),
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
