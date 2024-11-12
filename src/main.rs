#[path = "core/tasks.rs"]
mod tasks;

#[path = "core/manager.rs"]
mod manager;

#[path = "core/cli.rs"]
mod cli;

use cli::EventCLI;
use cli::{EventTypeCLI, CLI};

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let reminder_task_manager = manager::load_reminder_event_task_manager();
    let task_manager = Arc::new(Mutex::new(reminder_task_manager));

    // Start the notification task in the background
    let task_manager_clone = Arc::clone(&task_manager);
    tokio::spawn(async move {
        manager::ReminderTaskManager::start_notification_task(task_manager_clone).await;
    });

    // CLI loop
    loop {
        match CLI::get_event_cli() {
            Some(EventTypeCLI::Ok(mut value)) => {
                value.process_input(&mut task_manager.lock().unwrap());
            }
            None => {
                println!("Exiting CLI.");
                break;
            }
        }
    }
}
