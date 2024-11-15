use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Status {
    Completed,
    Active,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EventType {
    OneTime,
    Daily,
    Montly,
}

pub trait Tasks {
    fn summary(&self) -> String;
    fn update_status(&mut self, status: Status);
    fn notify(&self) -> bool;
    fn is_recurring(&self) -> bool;
    fn handle_update(&mut self);
}
