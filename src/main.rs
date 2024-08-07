use std::env;
use std::process;
use todo_app::{
    execute, get_input, handle_date, validate_priority, Command, Config, Priority, Task,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    // Create a list of tasks
    let mut tasks: Vec<Task> = Vec::new();

    // Handle the command
    match config.command.as_str() {
        "add" => {
            // TODO: Implement the add command
            // - Getting the task details from the user
            // - Creating a new task
            // - Adding the task to the list of tasks or saving it to a file
            let id: u128 = tasks.len() as u128 + 1;
            let title: String = get_input("Task title: ");
            let description: String = get_input("Task description: ");
            let done: bool = false;
            let priority: Priority = validate_priority(&get_input(
                "Priority (low, medium, or high): ",
            ))
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });
            let due_date = handle_date(&get_input("Due date (Example: 22-12-2024): "))
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
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
            let command = Command::AddTask(task);
            execute(command, &mut tasks);
            execute(Command::ListTasks, &mut tasks);
        }
        "remove" => {
            // TODO: Implement the remove command (Argument: task ID)
            if config.arguments.len() < 1 {
                eprintln!("Task ID is required for the remove command");
                process::exit(1);
            } else if config.arguments.len() > 1 {
                eprintln!("Too many arguments for the remove command");
                process::exit(1);
            }
            // - Validating the task ID
            let task_id: u128 = config
                .arguments
                .get(0)
                .unwrap()
                .parse::<u128>()
                .unwrap_or_else(|err| {
                    eprintln!("Invalid task ID: {}", err);
                    process::exit(1);
                });
            // - Removing the task from the list of tasks
            let command: Command = Command::RemoveTask(task_id);
            execute(command, &mut tasks);
            execute(Command::ListTasks, &mut tasks);
        }
        "done" => {
            // TODO: Implement the done command (Argument: task ID)
            if config.arguments.len() < 1 {
                eprintln!("Task ID is required for the done command");
                process::exit(1);
            } else if config.arguments.len() > 1 {
                eprintln!("Too many arguments for the done command");
                process::exit(1);
            }
            // - Validating the task ID
            let task_id: u128 = config
                .arguments
                .get(0)
                .unwrap()
                .parse::<u128>()
                .unwrap_or_else(|err| {
                    eprintln!("Invalid task ID: {}", err);
                    process::exit(1);
                });
            // - Changing the done status of the task
            let command: Command = Command::MarkAsDone(task_id);
            execute(command, &mut tasks);
            execute(Command::ListTasks, &mut tasks);
        }
        "priority" => {
            // TODO: Implement the priority command (Arguments: task ID, priority)
            if config.arguments.len() < 2 {
                eprintln!("Task ID and priority are required for the priority command");
                process::exit(1);
            } else if config.arguments.len() > 2 {
                eprintln!("Too many arguments for the priority command");
                process::exit(1);
            }
            // - Validating the task ID
            let task_id: u128 = config
                .arguments
                .get(0)
                .unwrap()
                .parse::<u128>()
                .unwrap_or_else(|err| {
                    eprintln!("Invalid task ID: {}", err);
                    process::exit(1);
                });
            // - Validating the priority
            let priority: &String = config.arguments.get(1).unwrap_or_else(|| {
                eprintln!("Priority is required for the priority command");
                process::exit(1);
            });
            // - Creating the priority enum from the string
            let priority: Priority = validate_priority(&priority.as_str()).unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });
            // - Changing the priority of the task
            let command: Command = Command::ChangePriority(task_id, priority);
            execute(command, &mut tasks);
            execute(Command::ListTasks, &mut tasks);
        }
        "list" => {
            // TODO: Implement the list command
            let command: Command = Command::ListTasks;
            execute(command, &mut tasks);
        }
        _ => {
            eprintln!("Invalid command: {}", config.command);
            process::exit(1);
        }
    }
}
