use chrono::naive::NaiveDate;
use std::env;
use std::io;
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

fn run(command: Command, task_list: &mut Vec<Task>) {
    match command {
        Command::AddTask(task) => {
            // Add the task to the list of tasks
            task_list.push(task);
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

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn validate_priority(priority: String) -> Result<Priority, &'static str> {
    match priority.as_str() {
        "low" => Ok(Priority::Low()),
        "medium" => Ok(Priority::Medium()),
        "high" => Ok(Priority::High()),
        _ => Err("Failed to validate priority"),
    }
}

fn handle_date(input: String) -> Result<NaiveDate, &'static str> {
    date = Ok(
        NaiveDate::parse_from_str(input.as_str(), "%d-%m-%Y").unwrap_or_else(|err| {
            return Err(err.to_string().as_str());
        }),
    )
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
        let command: &String = args.get(1).unwrap();
        let arguments: Vec<String> = args[2..].to_vec();
        Ok(Config { command, arguments })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    let mut tasks: Vec<Task> = Vec::new();
    match config.command.as_str() {
        "add" => {
            // TODO: Implement the add command
            // - Getting the task details from the user
            // - Creating a new task
            // - Adding the task to the list of tasks or saving it to a file
            let id: usize = tasks.len() + 1;
            let title: String = get_input("Task title: ");
            let description: String = get_input("Task description: ");
            let done: bool = false;
            let priority: Priority = validate_priority(get_input(
                "Priority (low, medium, or high)",
            ))
            .unwrap_or_else(|err| {
                eprintln!(err);
                process::exit(1);
            });
            let due_date = handle_date(get_input("Due date (22-12-2024)")).unwrap_or_else(|err| {
                eprintln!(err);
                process::exit(1);
            });
            let task: Task = Task {
                id: id,
                title: title,
                description: description,
                done: done,
                priority: priority,
                due_date: due_date,
            };
            command = Command::AddTask(task);
            run(command, &mut tasks);
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
