mod todo;

pub fn test_todo() {
    let mut todo = Todo::new(1, String::from("test"));
    todo.high();
    todo.toggle();
    todo.update(String::from("test2"));
    todo.low();
    todo.describe(true);

    let mut todo2 = Todo::new(2, String::from("test2"));
    todo2.high();
    todo2.toggle();
    todo2.update(String::from("test3"));
    todo2.low();
    todo2.describe(true);
}