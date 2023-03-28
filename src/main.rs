use std::{io::{self, Write}, process};

struct Todo {
    title: String,
    done: bool,
}

impl Todo {
    fn new(title: String) -> Self {
        Self {
            title,
            done: false,
        }
    }

    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

struct Todos {
    todos: Vec<Todo>
}

impl Todos {
    fn add(&mut self, title: String) {
        self.todos.push(Todo::new(title));
    }

    fn remove(&mut self, idx: usize) {
        self.todos.remove(idx - 1);
    }

    fn update(&mut self, idx: usize) {
        self.todos[idx - 1].toggle();
    }

    fn list(&self) {
        if self.todos.len() == 0 {
            println!("No todos");
        }

        for (idx, todo) in self.todos.iter().enumerate() {
            println!("{} - [{}] {}", idx + 1,if todo.done { "x" } else { " " }, todo.title);
        }
    }
}

fn main() {
    let mut todos = Todos{ todos: Vec::new() };

    loop {
        let inp = get_input();

        if inp.trim() == "exit" {
            process::exit(0);
        }

        if inp.trim() == "list" {
            todos.list();
            continue;
        }

        let split = inp.split(" ").collect::<Vec<&str>>();

        if split.len() >= 2 {
            let content = &split[1..].join(" ");
            
            match split[0] {
                "add" => todos.add(content.into()), 
                "remove" => todos.remove(content.parse().unwrap()),
                "toggle" => todos.update(content.parse().unwrap()),
                _ => print_help(),
            }
        } else {
            print_help();
        }
    }
}

fn get_input() -> String {
    let mut input: String = String::new();

    print!("TODO: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to readline");

    input.truncate(input.len() - 1);
    input.to_lowercase()
}

fn print_help() {
    println!("HELP\n------");
    println!("add <title> : add todo");
    println!("list : list all todos");
    println!("toggle <todo id> : change done to not done & vice versa of specific todo");
    println!("remove <todo id> : delete todo");
    println!("exit : quit program");
    println!("help : show this page");
}
