use chrono::naive::NaiveDate;
use std::env;
use std::process;

/// Represents a task in the todo app.
///
/// # Fields
///
/// - `id`: The unique identifier of the task.
/// - `title`: The title of the task.
/// - `description`: The description of the task.
/// - `done`: Indicates whether the task is done or not.
/// - `priority`: The priority of the task.
/// - `due_date`: The due date of the task.
///
/// # Example
///
/// ```
/// let task = Task {
///     id: 1,
///     title: String::from("Finish project"),
///     description: String::from("Complete the final tasks for the project"),
///     done: false,
///     priority: Priority::High,
///     due_date: NaiveDate::from_ymd(2022, 12, 31),
/// };
/// ```
struct Task {
    id: u128,
    title: String,
    description: String,
    done: bool,
    priority: Priority,
    due_date: NaiveDate,
}

enum Priority {
    Low,
    Medium,
    High,
}

enum Command {
    AddTask(Task),
    RemoveTask(u128),
    MarkAsDone(u128),
    ChangePriority(u128, Priority),
    ListTasks,
}

fn run(command: Command) {
    match command {
        Command::AddTask(task) => {
            // Add the task to the list of tasks
        }
        Command::RemoveTask(id) => {
            // Remove the task with the given ID from the list of tasks
        }
        Command::MarkAsDone(id) => {
            // Mark the task with the given ID as done
        }
        Command::ChangePriority(id, priority) => {
            // Change the priority of the task with the given ID
        }
        Command::ListTasks => {
            // List all tasks
        }
    }
}

struct Config {
    command: String,
    arguments: Vec<String>,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let command = String::from(args.get(1).unwrap().as_str());
        let arguments = args[2..].to_vec();
        Ok(Config { command, arguments })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    match config.command.as_str() {
        "add" => {
            // TODO: Implement the add command
            // - Getting the task details from the user
            // - Creating a new task
            // - Adding the task to the list of tasks or saving it to a file
        }
        "remove" => {
            // TODO: Implement the remove command (Argument: task ID)
        }
        "done" => {
            // TODO: Implement the done command (Argument: task ID)
        }
        "priority" => {
            // TODO: Implement the priority command (Arguments: task ID, priority)
            // - Validating the priority
            // - Creating the priority enum from the string
            // - Changing the priority of the task
        }
        "list" => {
            // TODO: Implement the list command
        }
        _ => {
            eprintln!("Invalid command: {}", config.command);
            process::exit(1);
        }
    }
}
