use std::env;
use std::process;
use todo_app::{load_tasks, run, Config, Task};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(args).unwrap_or_else(|_| {
        // Implement help message
        let help_message = "Usage: todo_app [command] [task_id] [priority] [due date]
        Commands:
        add: Add a new task
        list: List all tasks
        done task_id: Mark a task as done
        remove task_id: Remove a task
        priority task_id value: Change the priority of a task
        help: Display this message";
        eprintln!("{help_message}");
        process::exit(0);
    });

    // Create a list of tasks
    let mut tasks: Vec<Task> = load_tasks("tasks.json").unwrap_or_else(|err| {
        eprintln!("Error loading tasks: {}", err);
        process::exit(1);
    });
    run(config, &mut tasks);
}
