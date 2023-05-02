use std::io::{self, Write};

struct TodoApp {
    tasks: Vec<String>,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
        println!("Task added successfully.");
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task removed successfully.");
        } else {
            println!("Invalid task index.");
        }
    }

    fn print_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to display.");
        } else {
            for (i, task) in self.tasks.iter().enumerate() {
                println!("{}. {}", i + 1, task);
            }
        }
    }
}

fn main() {
    let mut todo_app = TodoApp::new();

    loop {
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. Print tasks");
        println!("4. Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                todo_app.add_task(input.trim().to_string());
            }
            2 => {
                print!("Enter task index: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = match input.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => continue,
                };
                todo_app.remove_task(index);
            }
            3 => {
                todo_app.print_tasks();
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }

        println!();
    }
}