use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::base::TaskManager;
use crate::tasks::base::{Status, Tasks};
use crate::tasks::reminder::Reminder;
pub struct ReminderTaskManager<T: Tasks> {
    pub tasks: HashMap<i32, T>,
    pub next_id: i32,
}

impl TaskManager<Reminder> for ReminderTaskManager<Reminder> {
    fn add(&mut self, task: Reminder) {
        let task_id = self.next_id;
        self.tasks.insert(task_id, task);
        self.next_id += 1;
    }

    fn delete(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
    }

    fn view(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            println!("\nCurrent Tasks:");
            println!("---------------------------------");

            for (task_id, task) in &self.tasks {
                println!("Task ID: {}", task_id);
                println!("Title: {}", task.title);
                println!("Description: {}", task.description);
                println!("Due Date: {}", task.due_date.format("%Y-%m-%d %H:%M:%S"));
                println!("Priority: {:?}", task.priority);
                println!("---------------------------------");
            }
        }
    }
}

impl ReminderTaskManager<Reminder> {
    pub async fn start_notification_task(task_manager: Arc<Mutex<Self>>) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;

            let mut task_manager = task_manager.lock().unwrap();

            for task in task_manager.tasks.values_mut() {
                if task.status == Status::Pending && task.should_notify() {
                    task.notify();
                    task.mark_complete();
                }
            }
        }
    }
}

pub fn load_reminder_event_task_manager() -> ReminderTaskManager<Reminder> {
    let (tasks, next_id) =
        <ReminderTaskManager<Reminder> as TaskManager<Reminder>>::default_values();
    let task_manager: ReminderTaskManager<Reminder> = ReminderTaskManager { tasks, next_id };
    task_manager
}
