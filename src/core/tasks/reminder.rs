use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::base::{Status, Tasks};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reminder {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
    pub status: Status,
}

impl Reminder {
    pub fn new(title: String, description: String, due_date: NaiveDateTime) -> Self {
        Self {
            title,
            description,
            due_date,
            status: Status::Active,
        }
    }
}
impl Tasks for Reminder {
    fn notify(&self) -> bool {
        matches!(self.status, Status::Active) && {
            let current_time = chrono::Local::now().naive_local();
            let notify_time = self.due_date - chrono::Duration::minutes(1);
            current_time >= notify_time && current_time < self.due_date
        }
    }

    fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    fn summary(&self) -> String {
        format!("Title: {}\nDescription: {}", self.title, self.description)
    }
}
