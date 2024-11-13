use super::reminder_cli::ReminderCLI;

pub enum EventTypeCLI {
    Ok(ReminderCLI),
}

pub trait EventCLI<M> {
    fn display_menu();
    fn process_input(&mut self, task_manager: &mut M);
}

pub struct CLI {}

impl CLI {
    pub fn welcome_message() {
        println!("\nSelect an option:");
        println!("1: Manage Reminders");
        println!("2: Manage Events");
        println!("3: Exit");
    }

    pub fn get_user_input() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    pub fn get_event_cli() -> Option<EventTypeCLI> {
        Self::welcome_message();
        let option = Self::get_user_input();

        match option.as_str() {
            "1" => Some(EventTypeCLI::Ok(ReminderCLI)),
            "3" => None,
            _ => None,
        }
    }
}
