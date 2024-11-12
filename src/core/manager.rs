use crate::tasks::{Reminder, Status, Tasks};
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait TaskManager<T: Tasks> {
    fn default_values() -> (HashMap<i32, T>, i32) {
        let tasks = HashMap::new();
        let next_id = 1;
        (tasks, next_id)
    }

    fn add(&mut self, task_name: T);
    fn delete(&mut self, task_id: i32);
    fn view(&mut self);
    fn complete(&mut self, task_id: i32);
}

pub struct ReminderTaskManager<T: Tasks> {
    pub tasks: HashMap<i32, T>,
    pub next_id: i32,
}

impl TaskManager<Reminder> for ReminderTaskManager<Reminder> {
    fn complete(&mut self, task_id: i32) {
        match self.tasks.get_mut(&task_id) {
            Some(task) => {
                task.mark_complete();
            }
            None => {}
        }
    }
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

impl ReminderTaskManager<Reminder> {}

pub fn load_reminder_event_task_manager() -> ReminderTaskManager<Reminder> {
    let (tasks, next_id) =
        <ReminderTaskManager<Reminder> as TaskManager<Reminder>>::default_values();
    let task_manager: ReminderTaskManager<Reminder> = ReminderTaskManager { tasks, next_id };
    task_manager
}
