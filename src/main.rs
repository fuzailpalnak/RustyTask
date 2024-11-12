#[path = "core/tasks.rs"]
mod tasks;

#[path = "core/manager.rs"]
mod manager;

#[path = "core/cli.rs"]
mod cli;

use cli::EventCLI;
use cli::{EventTypeCLI, CLI};

use std::sync::{Arc, Mutex};

fn main() {
    let mut reminder_task_manager = manager::load_reminder_event_task_manager();

    loop {
        match CLI::get_event_cli() {
            Some(EventTypeCLI::Ok(mut value)) => {
                value.process_input(&mut reminder_task_manager);
            }
            None => {
                println!("Exiting CLI.");
                break;
            }
        }
    }
}
