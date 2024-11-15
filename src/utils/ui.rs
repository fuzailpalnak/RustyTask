use notify_rust::error::Result;
use notify_rust::{Notification, Timeout};

pub fn sent_notification(message: &String, auto_close: bool) -> Result<()> {
    match auto_close {
        true => {
            Notification::new()
                .summary("Reminder")
                .body(&message)
                .icon("dialog-information")
                .show()?;
            Ok(())
        }
        false => {
            Notification::new()
                .summary("Reminder")
                .body(&message)
                .icon("dialog-information")
                .timeout(Timeout::Never)
                .show()?;
            Ok(())
        }
    }
}
