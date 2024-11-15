use std::collections::HashMap;

use crate::core::tasks::base::{Status, Tasks};
use crate::utils::ui;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;

pub struct TaskManager<T: Tasks> {
    pub tasks: HashMap<i32, T>,
    pub next_id: i32,
}

impl<T: Tasks> TaskManager<T> {
    pub fn get_tasks(&self) -> &HashMap<i32, T> {
        &self.tasks
    }
    pub fn add(&mut self, task: T) {
        let task_id = self.next_id;
        self.tasks.insert(task_id, task);
        self.next_id += 1;
    }

    pub fn complete(&mut self, task_id: i32) {
        match self.tasks.get_mut(&task_id) {
            Some(task) => task.update_status(Status::Completed),
            None => {}
        }
    }

    pub fn delete(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
    }

    pub fn monitor(&mut self) {
        let mut tasks_to_update = Vec::new();

        for (task_id, task) in self.tasks.iter_mut() {
            if task.notify() {
                match task.is_recurring() {
                    false => match ui::sent_notification(&task.summary(), false) {
                        Ok(_) => tasks_to_update.push(*task_id),
                        Err(e) => eprintln!("Failed to display notification: {}", e),
                    },
                    true => {
                        task.handle_update();
                    }
                }
            }
        }

        for task_id in tasks_to_update {
            self.complete(task_id);
        }
    }
}

pub fn load_task_manager<T: Tasks>() -> TaskManager<T> {
    let (tasks, next_id) = (HashMap::new(), 1);

    TaskManager { tasks, next_id }
}

pub async fn start_background_task<T>(task_manager: Arc<RwLock<TaskManager<T>>>)
where
    T: Tasks + Send + Sync,
{
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        {
            let mut task_manager = task_manager.write().await;
            task_manager.monitor();
        }
    }
}
