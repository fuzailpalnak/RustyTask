#[path = "core/tasks.rs"]
mod tasks;

#[path = "core/manager.rs"]
mod manager;

#[path = "core/cli.rs"]
mod cli;

use cli::EventCLI;
use cli::{EventTypeCLI, CLI};

fn main() {
    loop {
        match CLI::get_event_cli() {
            Some(EventTypeCLI::Ok(mut value)) => {
                let mut task_manager = value.load_task_manger();
                value.process_input(&mut task_manager);
            }
            None => {
                println!("Exiting CLI.");
                break;
            }
        }
    }
}
