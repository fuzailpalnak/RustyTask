use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Status {
    Completed,
    Active,
}

pub trait Tasks {
    fn summary(&self) -> String;
    fn update_status(&mut self, status: Status);
    fn notify(&self) -> bool;
}
