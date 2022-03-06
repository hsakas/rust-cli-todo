mod utils;
use utils::todo::{Todo, TodoList};

fn main(){
    let mut todo_list = TodoList::new();
    todo_list.add(Todo::new(1, String::from("Buy milk")));
    todo_list.add(Todo::new(2, String::from("Buy eggs")));
    todo_list.add(Todo::new(3, String::from("Buy bread")));
    todo_list.add(Todo::new(4, String::from("Buy coffee")));
    todo_list.add(Todo::new(5, String::from("Buy tea")));
    todo_list.add(Todo::new(6, String::from("Buy chocolate")));
    todo_list.add(Todo::new(7, String::from("Buy milk")));
    todo_list.add(Todo::new(8, String::from("Buy eggs")));
    todo_list.add(Todo::new(9, String::from("Buy bread")));

    todo_list.describe(true);
    todo_list.toggle(1);
    todo_list.high(1);
    todo_list.remove(9);
    todo_list.remove(2);
    print!("\n");
    todo_list.describe(true);
    
}