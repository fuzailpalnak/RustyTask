use notify_rust::Notification;

pub fn sent_notification(message: &String) {
    Notification::new()
        .summary("Reminder")
        .body(&message)
        .icon("dialog-information")
        .show()
        .unwrap();
}
