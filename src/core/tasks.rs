use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

pub trait Tasks<T> {
    fn mark_complete(&mut self);
    fn update_priority(&mut self, priority: Priority);
    fn update_status(&mut self, status: Status);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    title: String,
    description: String,
    due_date: NaiveDateTime,
    priority: Priority,
    status: Status,
}

impl Reminder {
    pub fn new(
        title: String,
        description: String,
        due_date: NaiveDateTime,
        priority: Priority,
    ) -> Self {
        Self {
            title,
            description,
            due_date,
            priority,
            status: Status::Pending,
        }
    }
}
impl<T> Tasks<T> for Reminder {
    fn mark_complete(&mut self) {
        self.status = Status::Completed;
    }

    fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    fn update_status(&mut self, status: Status) {
        self.status = status;
    }
}
