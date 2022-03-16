extern crate utils;
use utils::todo::Todo;

fn main(){
    let mut todo = Todo::new(1, String::from("Learn Rust"));
    todo.high();
    todo.medium();
    todo.low();
    todo.toggle();
    todo.update(String::from("Learn Rust"));
    todo.describe(true);

    let mut todo = Todo::new(2, String::from("Learn Rust"));
    todo.high();
    todo.medium();
    todo.low();
    todo.toggle();

    let mut todo = Todo::new(3, String::from("Learn Rust"));
    todo.high().toggle().describe(true);
}