use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::base::{EventType, Status, Tasks};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reminder {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
    pub status: Status,
    pub event_type: EventType,
}

impl Reminder {
    pub fn new(title: String, description: String, due_date: NaiveDateTime) -> Self {
        Self {
            title,
            description,
            due_date,
            status: Status::Active,
            event_type: EventType::OneTime,
        }
    }
}
impl Tasks for Reminder {
    fn handle_update(&mut self) {
        let current_time = chrono::Local::now().naive_local();

        match self.event_type {
            EventType::Daily => {
                self.due_date = current_time + chrono::Duration::days(1);
            }
            EventType::Montly => {
                self.due_date = current_time + chrono::Duration::days(30);
            }
            _ => {}
        }
    }

    fn is_recurring(&self) -> bool {
        match self.event_type {
            EventType::OneTime => false,
            _ => true,
        }
    }
    fn notify(&self) -> bool {
        matches!(self.status, Status::Active) && {
            let current_time = chrono::Local::now().naive_local();
            let notify_time = self.due_date - chrono::Duration::minutes(15);
            current_time >= notify_time && current_time < self.due_date
        }
    }

    fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    fn summary(&self) -> String {
        format!(
            "🚨 Reminder: {}\n🔔 Don't forget to: {}",
            self.title, self.description
        )
    }
}
