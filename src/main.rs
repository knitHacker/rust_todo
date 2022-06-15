
use std::str::FromStr;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct CmdParseError {
    details: String
}

impl CmdParseError {
    fn new(msg: String) -> CmdParseError {
        CmdParseError{details: msg}
    }
}


impl fmt::Display for CmdParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CmdParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
  

struct ListItem {
    description: String,
    completed: bool,
}

enum ListType {
    All,
    Open,
    Done,
}

enum Command {
    Add,
    Delete,
    Complete,
    List (ListType),
    Help,
    Quit,
}

impl FromStr for Command {
    type Err = CmdParseError;

    fn from_str(s: &str) -> Result<Self, CmdParseError> {
        let mut iter = s.split_ascii_whitespace();
        match iter.next() {
            Some(cmd) => {
                if cmd == "help" || cmd == "h" {
                    return Ok(Command::Help);
                } else if cmd == "quit" || cmd == "q" {
                    return Ok(Command::Quit);
                } else if cmd == "add" || cmd == "a" {
                    return Ok(Command::Add);
                } else if cmd == "delete" || cmd == "d" {
                    return Ok(Command::Delete);
                } else if cmd == "complete" || cmd == "x" {
                    return Ok(Command::Complete);
                } else if cmd == "list" || cmd == "l" {
                    match iter.next() {
                        None => return Ok(Command::List(ListType::All)),
                        Some(t) => {
                            if t == "all" || t == "a" {
                                return Ok(Command::List(ListType::All));
                            } else if t == "done" || t == "d" {
                                return Ok(Command::List(ListType::Done));
                            } else if t == "open" || t == "o" {
                                return Ok(Command::List(ListType::Open));
                            }
                            return Err(CmdParseError::new(format!("unknown list argument: {}", t)));
                        },
                    }
                }
            }
            None => (),
        }
        Err(CmdParseError::new(format!("unknown command: {}", s)))
    }
}

fn parse_command() -> Result<Command, CmdParseError> {
    println!("\nPlease enter a command");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    Command::from_str(line.trim())
}

fn main() {
    println!("Todo list!");
    let mut todos: Vec<ListItem> = vec![];
    loop {
        match parse_command() {
            Err(err) => println!("{}", err),
            Ok(cmd) => match cmd {
                Command::Add => add_new_todo(&mut todos),
                Command::Delete => remove_todo(&mut todos),
                Command::List(list_type) => list_todos(list_type, &todos),
                Command::Complete => complete_todo(&mut todos),
                Command::Quit => break,
                _ => println!("Command is still being implemented")
            }
        }
    }
}

fn list_todos(list_type: ListType, todos: &Vec<ListItem>) {
    for (i, item) in todos.iter().enumerate() {
        println!("{}: {} - {}", i + 1, item.description, if item.completed { "X" } else { "O" });
    }
}


fn complete_todo(todos: &mut Vec<ListItem>) {
    println!("Enter the number of the item you want to complete");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let index_r = usize::from_str_radix(line.trim(), 10);
    match index_r {
        Err(err) => println!("Please enter a valid index: {}", err),
        Ok(index) => {
            if index < todos.len() {
                todos[index-1].completed = true;
                println!("'{}' marked as completed", todos[index-1].description);
            } else {
                println!("There are only {} todo items!", todos.len());
            }
        }
    }
   
}

fn add_new_todo(todos: &mut Vec<ListItem>) {
    println!("Please type in a new todo item");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("Adding '{}' to the todo list", line.trim());
    let new_item = line.trim().to_string();
    todos.push(ListItem { description: new_item, completed: false });
}

fn remove_todo(todos: &mut Vec<ListItem>) {
    println!("Enter the number of the item you want to remove");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let index_r = usize::from_str_radix(line.trim(), 10);
    match index_r {
        Err(err) => println!("Please enter a valid index: {}", err),
        Ok(index) => {
            if index < todos.len() {
                let removed = todos.remove(index-1);
                println!("Removed item '{}'", removed.description);
            } else {
                println!("There are only {} todo items!", todos.len());
            }
        }
    }
}
