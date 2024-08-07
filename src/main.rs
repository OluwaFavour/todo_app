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
