mod utils;
use utils::todo::Todo as Todo;

fn fake_todo() -> Vec<Todo>{
    let mut todos = Vec::new();
    todos.push(Todo::new(1, String::from("Buy milk")));
    todos.push(Todo::new(2, String::from("Buy bread")));
    todos.push(Todo::new(3, String::from("Buy coffee")));
    todos
}


fn main(){
    let mut todos: Vec<Todo> = fake_todo();

    todos.push(Todo::new(4, String::from("Buy milk")));
    for todo in todos.iter(){
        todo.describe(true);
    }

    todos.append(&mut fake_todo());
    for todo in todos.iter(){
        todo.describe(true);
    }

}