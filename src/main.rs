#[path = "core/tasks.rs"]
mod tasks;

#[path = "core/manager.rs"]
mod manager;

use chrono::NaiveDate;
use manager::TaskManager;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    name: String,
    age: u32,
}

fn main() {
    let due_date = NaiveDate::from_ymd_opt(2024, 11, 10).unwrap();

    let reminder = tasks::Reminder::new(
        "Study Rust".to_string(),
        "Complete Rust book".to_string(),
        due_date,
        tasks::Priority::High,
    );
    let mut task_manager: manager::ReminderTaskManager<tasks::Reminder> =
        manager::ReminderTaskManager::new();

    task_manager.add_task(reminder);

    task_manager.complete(1);
}
