use std::env;
use std::process;
use todo_app::{
    execute, get_input, handle_date, run, validate_priority, Command, Config, Priority, Task,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    // Create a list of tasks
    let mut tasks: Vec<Task> = Vec::new();
    run(config, tasks);
}
