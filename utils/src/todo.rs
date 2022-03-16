struct Priority {
    value: i32,
}

impl Priority {
    fn high(&mut self) {
        self.value = 2;
    }

    fn medium(&mut self) {
        self.value = 1;
    }

    fn low(&mut self) {
        self.value = 0;
    }
}

pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
    priority: Priority,
    tag: String,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
            priority: Priority { value: 0 },
            tag: String::from(""),
        }
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self) -> &mut Self {
        self.completed = !self.completed;
        self
    }

    #[allow(dead_code)]
    pub fn update(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    #[allow(dead_code)]
    pub fn high(&mut self) -> &mut Self {
        self.priority.high();
        self
    }

    #[allow(dead_code)]
    pub fn medium(&mut self) -> &mut Self {
        self.priority.medium();
        self
    }

    #[allow(dead_code)]
    pub fn low(&mut self) -> &mut Self {
        self.priority.low();
        self
    }

    #[allow(dead_code)]
    pub fn set_tag(&mut self, tag: String) -> &mut Self {
        self.tag = tag;
        self
    }

    #[allow(dead_code)]
    pub fn describe(&self, verbose: bool) {
        let mut description = String::from("");
        if verbose {
            description.push_str(&format!("{} ", self.id));
        }
        description.push_str(&format!("{} ", self.text));
        if self.completed {
            description.push_str("✔ ");
        } else {
            description.push_str("✘ ");
        }

        if self.priority.value == 2 {
            description.push_str("⚡️ ");
        } else if self.priority.value == 1 {
            description.push_str("⚠️ ");
        } else {
            description.push_str("⚖️ ");
        }

        description.push_str(&format!("#{} ", self.tag));
        println!("{}", description);
    }
}

pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    #[allow(dead_code)]
    pub fn describe(&self, verbose: bool) {
        for todo in self.todos.iter() {
            todo.describe(verbose);
        }
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.toggle();
            }
        }
    }

    #[allow(dead_code)]
    pub fn high(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.high();
            }
        }
    }

    #[allow(dead_code)]
    pub fn medium(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.medium();
            }
        }
    }

    #[allow(dead_code)]
    pub fn low(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.low();
            }
        }
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, id: i32) {
        self.todos.retain(|todo| todo.id != id);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.todos.clear();
    }

    #[allow(dead_code)]
    pub fn retain(&mut self, completed: bool) -> &mut Self {
        if completed {
            self.todos.retain(|todo| todo.completed);
        } else {
            self.todos.retain(|todo| !todo.completed);
        }
        self
    }

    // get all todos which are completed
    // the original todos are not modified 
    //this operation just filters the todos
    #[allow(dead_code)]
    pub fn get_completed(&self) -> Vec<&Todo> {
        let mut completed = Vec::new();
        for todo in self.todos.iter() {
            if todo.completed {
                completed.push(todo.clone());
            }
        }
        completed
    }

    #[allow(dead_code)]
    pub fn get_incomplete(&self) -> Vec<&Todo> {
        let mut incomplete = Vec::new();
        for todo in self.todos.iter() {
            if !todo.completed {
                incomplete.push(todo.clone());
            }
        }
        incomplete
    }
}

