use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

pub trait Tasks {
    fn mark_complete(&mut self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
    pub priority: Priority,
    pub status: Status,
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
impl Tasks for Reminder {
    fn mark_complete(&mut self) {
        self.status = Status::Completed;
    }
}
