use chrono::naive::NaiveDate;
use std::io::{self, Write};
use std::process;

/// Gets input from the user with the given prompt.
///
/// # Arguments
///
/// - `prompt`: The prompt to display to the user.
///
/// # Returns
///
/// The input entered by the user.
///
/// # Example
///
/// ```
/// use todo_app::get_input;
///
/// let title = get_input("Task title: ");
/// ```
pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

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
/// use chrono::naive::NaiveDate;
/// use todo_app::{Task, Priority};
///
/// let task = Task {
///     id: 1,
///     title: String::from("Finish project"),
///     description: String::from("Complete the final tasks for the project"),
///     done: false,
///     priority: Priority::High,
///     due_date: NaiveDate::from_ymd(2022, 12, 31),
/// };
/// ```
pub struct Task {
    pub id: u128,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub priority: Priority,
    pub due_date: NaiveDate,
}

/// Represents the priority of a task.
/// The priority can be low, medium, or high.
/// The priority is used to determine the importance of the task.
/// The priority can be changed by the user.
///
/// # Example
///
/// ```
/// use todo_app::Priority;
/// let priority = Priority::High;
/// ```
#[derive(Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/// Represents the commands that can be executed in the todo app.
/// The commands are used to perform actions such as adding a task, removing a task, marking a task as done, changing the priority of a task, and listing all tasks.
///
/// # Example
///
/// ```
/// use chrono::naive::NaiveDate;
/// use todo_app::{Command, Task, Priority};
///
/// let task: Task = Task {
///     id: 1,
///     title: String::from("Finish project"),
///     description: String::from("Complete the final tasks for the project"),
///     done: false,
///     priority: Priority::High,
///     due_date: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
/// };
/// let command = Command::AddTask(task);
/// ```
pub enum Command {
    AddTask(Task),
    RemoveTask(u128),
    MarkAsDone(u128),
    ChangePriority(u128, Priority),
    ListTasks,
}

/// Executes the given command with the given list of tasks.
/// The function performs the action specified by the command on the list of tasks.
///
/// # Arguments
///
/// - `command`: The command to run.
/// - `task_list`: The list of tasks to perform the action on.
///
/// # Example
///
/// ```
/// use chrono::naive::NaiveDate;
/// use todo_app::{execute, Command, Task, Priority};
///
/// let task: Task = Task {
///     id: 1,
///     title: String::from("Finish project"),
///     description: String::from("Complete the final tasks for the project"),
///     done: false,
///     priority: Priority::High,
///     due_date: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
/// };
/// let mut tasks: Vec<Task> = Vec::new();
/// execute(Command::AddTask(task), &mut tasks);
/// ```
pub fn execute(command: Command, task_list: &mut Vec<Task>) {
    match command {
        Command::AddTask(task) => {
            // Add the task to the list of tasks
            task_list.push(task);
        }
        Command::RemoveTask(id) => {
            // Remove the task with the given ID from the list of tasks
            let index: Option<usize> = task_list.iter().position(|task| task.id == id);
            if let Some(index) = index {
                task_list.swap_remove(index);
            } else {
                println!("Task not found");
            }
        }
        Command::MarkAsDone(id) => {
            // Mark the task with the given ID as done
            let task = task_list.iter_mut().find(|task| task.id == id);
            if let Some(task) = task {
                task.done = true;
            } else {
                println!("Task not found");
            }
        }
        Command::ChangePriority(id, priority) => {
            // Change the priority of the task with the given ID
            let task = task_list.iter_mut().find(|task| task.id == id);
            if let Some(task) = task {
                task.priority = priority;
            } else {
                println!("Task not found");
            }
        }
        Command::ListTasks => {
            // List all tasks
            for task in task_list {
                println!("Task ID: {}", task.id);
                println!("Title: {}", task.title);
                println!("Description: {}", task.description);
                println!("Done: {}", task.done);
                println!("Priority: {:?}", task.priority);
                println!("Due Date: {}", task.due_date);
            }
        }
    }
}

/// Validates the given priority string.
///
/// # Arguments
///
/// - `priority`: The priority string to validate.
///
/// # Returns
///
/// The priority enum if the string is valid, otherwise an error message.
///
/// # Example
///
/// ```
/// use todo_app::validate_priority;
///
/// let priority = validate_priority("high").unwrap();
/// ```
pub fn validate_priority(priority: &str) -> Result<Priority, &'static str> {
    match priority {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        _ => Err("Failed to validate priority"),
    }
}

/// Handles the date string and converts it to a NaiveDate.
///
/// # Arguments
///
/// - `date`: The date string to handle.
///
/// # Returns
///
/// The NaiveDate if the date string is valid, otherwise an error message.
///
/// # Example
///
/// ```
/// use todo_app::handle_date;
/// let date = handle_date("22-12-2024").unwrap();
/// ```
pub fn handle_date(date: &str) -> Result<NaiveDate, &'static str> {
    NaiveDate::parse_from_str(date, "%d-%m-%Y").map_err(|_| "Invalid date format")
}

/// Represents the configuration of the todo app.
/// The configuration includes the command to execute and the arguments for the command.
///
/// # Example
///
/// ```
/// use todo_app::Config;
///
/// let config = Config {
///    command: String::from("add"),
///   arguments: vec![String::from("Finish project")],
/// };
/// ```
pub struct Config {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Config {
    /// Builds the configuration from the given arguments.
    ///
    /// # Arguments
    ///
    /// - `args`: The arguments to parse.
    ///
    /// # Returns
    ///
    /// The configuration if the arguments are valid, otherwise an error message.
    ///
    /// # Example
    ///
    /// ```
    /// use todo_app::Config;
    ///
    /// let args: Vec<String> = vec!["todo".to_string(), "add".to_string(), "Finish project".to_string()];
    /// let config = Config::build(args).unwrap();
    /// ```
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let command: String = args.get(1).unwrap().to_string();
        let arguments: Vec<String> = args[2..].to_vec();
        Ok(Config { command, arguments })
    }
}

/// Runs the todo app with the given configuration and list of tasks.
/// The function executes the command specified in the configuration on the list of tasks.
/// The function handles the add, remove, done, priority, and list commands.
///
/// # Arguments
///
/// - `config`: The configuration of the todo app.
/// - `tasks`: The list of tasks to perform the action on.
///
/// # Example
///
/// ```
/// use todo_app::{run, Config, Task};
/// let config = Config {
///    command: String::from("done"),
///  arguments: vec![String::from("1")],
/// };
/// let tasks: Vec<Task> = Vec::new();
/// run(config, tasks);
/// ```
pub fn run(config: Config, mut tasks: Vec<Task>) {
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
