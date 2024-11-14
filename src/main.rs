mod core;
mod utils;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::cli::CLI;
use crate::core::manager;

#[tokio::main]
async fn main() {
    let reminder_task_manager = manager::load_task_manager();
    let task_manager = Arc::new(RwLock::new(reminder_task_manager));

    let task_manager_clone = Arc::clone(&task_manager);
    tokio::spawn(async move {
        manager::start_background_task(task_manager_clone).await;
    });
    if let Err(err) = CLI::run(&task_manager).await {
        println!("{}", err);
    }
}
