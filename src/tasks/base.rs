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
