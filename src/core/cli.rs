use chrono::NaiveDateTime;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::tasks::reminder::Reminder;
use crate::manager::TaskManager;
use crate::utils::ui;
pub struct CLI;

impl CLI {
    pub fn display_menu() {
        println!("\nReminder Management:");
        println!("1: Add Task");
        println!("2: View Tasks");
        println!("3: Delete Task");
        println!("4: Exit");
    }

    pub fn get_user_input() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    pub async fn run(task_manager: &Arc<RwLock<TaskManager<Reminder>>>) -> Result<(), String> {
        loop {
            CLI::display_menu();
            let option = CLI::get_user_input();

            match option.as_str() {
                "1" => CLI::add(task_manager).await?,
                "2" => CLI::view(task_manager).await?,
                "3" => CLI::delete(task_manager).await?,
                "4" => {
                    return Err("exiting program.".to_string());
                }
                _ => {
                    return Err("Invalid option, exiting program.".to_string());
                }
            }
        }
    }

    pub fn get_reminder() -> Result<Reminder, String> {
        println!("Enter task title:");
        let title = CLI::get_user_input();
        println!("Enter task description:");
        let description = CLI::get_user_input();
        println!("Enter the due date and time (YYYY-MM-DD HH:MM):");
        let due_datetime_input = CLI::get_user_input();
        let due_datetime =
            match NaiveDateTime::parse_from_str(&due_datetime_input, "%Y-%m-%d %H:%M") {
                Ok(datetime) => datetime,
                Err(_) => {
                    return Err("Invalid date-time format. Please use YYYY-MM-DD HH:MM.".to_string())
                }
            };

        let reminder = Reminder::new(title, description, due_datetime);
        Ok(reminder)
    }

    pub fn get_id_from_user_prompt() -> Result<i32, ()> {
        println!("Enter task ID to delete:");
        CLI::get_user_input().parse().map_err(|_| ())
    }

    async fn delete(task_manager: &Arc<RwLock<TaskManager<Reminder>>>) -> Result<(), String> {
        match CLI::get_id_from_user_prompt() {
            Ok(id) => {
                let mut task_manager = task_manager.write().await;
                task_manager.delete(id);

                match ui::sent_notification(&String::from("Feature Not Supported."), true) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Failed to delete reminder.".to_string()),
                }
            }
            Err(_) => {
                println!("Failed to delete reminder.");
                Err("Failed to delete reminder.".to_string())
            }
        }
    }

    async fn view(task_manager: &Arc<RwLock<TaskManager<Reminder>>>) -> Result<(), String> {
        let task_manager = task_manager.read().await;
        println!("\nCurrent Tasks:");
        println!("---------------------------------");

        for (task_id, task) in task_manager.get_tasks() {
            println!("Task ID: {}", task_id);
            println!("Title: {}", task.title);
            println!("Description: {}", task.description);
            println!("Due Date: {}", task.due_date.format("%Y-%m-%d %H:%M:%S"));
            println!("---------------------------------");
        }

        Ok(())
    }

    async fn add(task_manager: &Arc<RwLock<TaskManager<Reminder>>>) -> Result<(), String> {
        match CLI::get_reminder() {
            Ok(reminder) => {
                let mut task_manager = task_manager.write().await;
                task_manager.add(reminder);

                match ui::sent_notification(
                    &String::from("✅ Your reminder has been successfully added! You're all set!"),
                    true,
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to create reminder due to {}.", e)),
                }
            }
            Err(_) => {
                println!("Failed to add reminder.");
                Err("Failed to add reminder.".to_string())
            }
        }
    }
}
