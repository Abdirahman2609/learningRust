use std::io;

struct Task {
    description: String,
    completed: bool,
}

struct TodoList {
    numtasks: usize,
    list: Vec<Task>,
}

impl Task {
    fn get_description(&self) -> &str {
        return &self.description;
    }

    fn close(&mut self) {
        self.completed = true;
    }
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            numtasks: 0,
            list: Vec::new(),
        }
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.list.get_mut(index) {
            task.close();
            println!("Task {} is complete.", index);
        } else {
            println!("Could not find task at the index given.");
        }
    }

    fn view_tasks(&self) {
        for num in 0..self.numtasks {
            if let Some(task) = self.list.get(num) {
                println!("Task {}: {}", num, task.get_description());
            }
        }
    }

    fn add_task(&mut self, task: Task) {
        self.list.push(task);
        self.numtasks += 1;
    }

    fn delete_task(&mut self, index: usize) {
        self.list.remove(index);
        self.numtasks = self.numtasks - 1;
    }
}

fn main() {
    let mut todolist = TodoList::new();
    let mut exit = false;

    while !exit {
        println!(
            "(1) Add a task, (2) View Tasks, (3) Delete a Task, (4) Mark a task done, (5) Exit"
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(value) = input.trim().parse::<isize>() {
            match value {
                1 => {
                    println!("Write a description for you task");
                    let mut desc = String::new();
                    io::stdin().read_line(&mut desc).unwrap();
                    let newtask = Task {
                        description: desc,
                        completed: false,
                    };
                    todolist.add_task(newtask);
                }
                2 => {
                    todolist.view_tasks();
                }
                3 => {
                    println!("What index is the task you want to delete?");
                    let mut result = String::new();
                    io::stdin().read_line(&mut result).unwrap();
                    todolist.delete_task(result.trim().parse().unwrap());
                }
                4 => {
                    println!("What is the index of the task you want marked as done?");
                    let mut index = String::new();
                    io::stdin().read_line(&mut index).unwrap();
                    if let Ok(index) = index.trim().parse::<usize>() {
                        todolist.complete_task(index);
                    }
                }
                5 => {
                    exit = true;
                    break;
                }
                _ => println!("Has to be a number in the range 1 to 5"),
            }
        } else {
            println!("Input was invalid");
        }
    }
}
