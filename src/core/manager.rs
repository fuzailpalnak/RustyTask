use crate::tasks::{Priority, Reminder, Status, Tasks};
use std::collections::HashMap;

pub trait TaskManager<T: Tasks> {
    fn default_values() -> (HashMap<i32, T>, i32) {
        let tasks = HashMap::new();
        let next_id = 1;
        (tasks, next_id)
    }

    fn add_task(&mut self, task_name: T);
    fn delete_task(&mut self, task_id: i32);
    fn view_tasks(&mut self);
}

pub struct ReminderTaskManager<T: Tasks> {
    tasks: HashMap<i32, T>,
    next_id: i32,
}

impl TaskManager<Reminder> for ReminderTaskManager<Reminder> {
    fn add_task(&mut self, task: Reminder) {
        let task_id = self.next_id;
        self.tasks.insert(task_id, task);
        self.next_id += 1;
    }

    fn delete_task(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
    }

    fn view_tasks(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            println!("\nCurrent Tasks:");
            for (task_id, task) in &self.tasks {
                println!("{:?}", task);
            }
            println!("---------------------------------");
        }
    }
}

impl ReminderTaskManager<Reminder> {
    pub fn new() -> Self {
        let (tasks, next_id) =
            <ReminderTaskManager<Reminder> as TaskManager<Reminder>>::default_values();
        ReminderTaskManager { tasks, next_id }
    }

    pub fn complete(&mut self, task_id: i32) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.mark_complete();
        }
    }

    pub fn update_priority(&mut self, task_id: i32, priority: Priority) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.update_priority(priority);
        }
    }

    pub fn update_status(&mut self, task_id: i32, status: Status) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.update_status(status);
        }
    }
}
