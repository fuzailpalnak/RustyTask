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

    pub fn should_notify(&self) -> bool {
        let now = chrono::Local::now().naive_local();
        let notify_time = self.due_date - chrono::Duration::minutes(15);
        println!("{:?}", notify_time);
        println!("{:?}", now);
        now >= notify_time && now < self.due_date
    }

    pub fn notify(&self) {
        println!("Reminder: '{}' is due in 15 minutes!", self.title);
    }
}
impl Tasks for Reminder {
    fn mark_complete(&mut self) {
        self.status = Status::Completed;
    }
}
