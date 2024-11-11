use crate::tasks::{Priority, Reminder, Status, Tasks};
use std::collections::HashMap;

pub trait TaskManager<T> {
    fn default_values() -> (HashMap<i32, T>, i32) {
        let tasks = HashMap::new();
        let next_id = 1;
        (tasks, next_id)
    }

    fn add_task(&mut self, task_name: T);
}
pub struct ReminderTaskManager<Reminder> {
    tasks: HashMap<i32, Reminder>,
    next_id: i32,
}

impl<Reminder> TaskManager<Reminder> for ReminderTaskManager<Reminder> {
    fn add_task(&mut self, task: Reminder) {
        let task_id = self.next_id;
        self.tasks.insert(task_id, task);
        self.next_id += 1;
    }
}

impl<T> ReminderTaskManager<T>
where
    T: Tasks<Reminder>,
{
    pub fn new() -> Self {
        let (tasks, next_id) = <ReminderTaskManager<T> as TaskManager<T>>::default_values();
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

    pub fn delete_task(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
    }
}
