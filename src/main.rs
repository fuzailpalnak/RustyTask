#[path = "core/tasks.rs"]
mod tasks;

#[path = "core/manager.rs"]
mod manager;

#[path = "core/cli/base.rs"]
mod cli;

#[path = "core/cli/reminder.rs"]
mod reminder_cli;

use cli::EventCLI;
use manager::ReminderTaskManager;
use reminder_cli::ReminderCLI;
use tasks::Reminder;

fn main() {
    let mut task_manager: ReminderTaskManager<Reminder> = ReminderTaskManager::new();

    loop {
        let option = cli::welcome_message();
        match option.as_str() {
            "1" => {
                let mut reminder_cli = ReminderCLI;
                reminder_cli.process_input(&mut task_manager);
            }
            "3" => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}
