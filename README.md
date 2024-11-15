# RustyTask

**RustyTask** is a command-line task manager built in **Rust**, designed to help you organize, prioritize, and track your daily tasks with ease. With features like recurring tasks, background notifications, and a clean modular architecture, RustyTask is perfect for boosting your productivity while exploring the power of Rust.

---

## Features

- 📝 **Add and Manage Tasks**: Add tasks with titles, descriptions, and due dates.
- 🔔 **Background Notifications**: Receive alerts 15 minutes before tasks are due.
- ♻️ **Recurring Events**: Support for daily and monthly recurring tasks.
- ⚡ **Lightweight and Efficient**: Built using asynchronous Rust for smooth performance.
- 🔒 **Modular Design**: Easy to extend and maintain.

---

## How It Works

1. **Task Creation**: Tasks (like reminders) are stored in a centralized `TaskManager`.
2. **Background Monitoring**: A background task checks for upcoming events every second and triggers notifications when needed.
3. **Recurring Tasks**: Automatically updates recurring tasks after completion.
4. **CLI Interface**: Interact with RustyTask using a simple and intuitive CLI.

---

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/fuzailpalnak/rustytask.git
   cd rustytask
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   ./target/release/rustytask
   ```

---

## Usage

### Adding a Task
Run the CLI and follow the prompts to add tasks:

```bash
$ ./rustytask
```

Provide the required information:
- **Title**: A short description of the task.
- **Description**: Optional details about the task.
- **Due Date**: When the task is due.

---

### Background Notifications
RustyTask runs a background task to monitor your tasks and notify you when they’re due.

- Notifications appear 15 minutes before the task’s due time.
- Completed tasks are automatically marked or deleted based on their type.

---

## Project Structure

```
src/
├── core/
│   ├── cli.rs          # CLI module for user interaction
│   ├── manager.rs      # Manages tasks and background monitoring
│   ├── tasks/
│       ├── base.rs     # Trait and base logic for tasks
│       ├── reminder.rs # Reminder implementation of a task
├── utils/
│   ├── ui.rs           # Utility functions for notifications
├── main.rs             # Application entry point
```

---

## Contributing

Contributions are welcome! If you’d like to improve RustyTask or add new features:
1. Fork the repository.
2. Create a new branch for your feature/bugfix.
3. Submit a pull request.

---

## Contact

If you have questions or feedback, feel free to reach out at `fuzailpalnak@outlook.com`.

Enjoy using **RustyTask**! 🚀