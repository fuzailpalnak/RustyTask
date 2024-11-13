use std::collections::HashMap;

use crate::tasks::base::Tasks;
pub trait TaskManager<T: Tasks> {
    fn default_values() -> (HashMap<i32, T>, i32) {
        let tasks = HashMap::new();
        let next_id = 1;
        (tasks, next_id)
    }

    fn add(&mut self, task_name: T);
    fn delete(&mut self, task_id: i32);
    fn view(&self);
}
