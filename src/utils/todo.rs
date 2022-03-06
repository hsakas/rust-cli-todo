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

pub struct TodoList{
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn describe(&self, verbose: bool) {
        for todo in self.todos.iter() {
            todo.describe(verbose);
        }
    }

    pub fn toggle(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.toggle();
            }
        }
    }

    pub fn high(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.high();
            }
        }
    }

    pub fn medium(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.medium();
            }
        }
    }

    pub fn low(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.low();
            }
        }
    }

    pub fn remove(&mut self, id: i32) {
        self.todos.retain(|todo| todo.id != id);
    }

    pub fn clear(&mut self) {
        self.todos.clear();
    }

    pub fn filter(&self, tag: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.tag == tag).collect()
    }

    pub fn filter_completed(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed).collect()
    }

    pub fn filter_incomplete(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed).collect()
    }

    pub fn filter_priority(&self, priority: i32) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.priority.value == priority).collect()
    }

    pub fn filter_tag(&self, tag: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.tag == tag).collect()
    }

    pub fn filter_text(&self, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.text.contains(text)).collect()
    }

    pub fn filter_completed_text(&self, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed && todo.text.contains(text)).collect()
    }

    pub fn filter_incomplete_text(&self, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed && todo.text.contains(text)).collect()
    }

    pub fn filter_priority_text(&self, priority: i32, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.priority.value == priority && todo.text.contains(text)).collect()
    }

    pub fn filter_tag_text(&self, tag: &str, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.tag == tag && todo.text.contains(text)).collect()
    }

    pub fn filter_completed_priority_text(&self, priority: i32, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed && todo.priority.value == priority && todo.text.contains(text)).collect()
    }

    pub fn filter_incomplete_priority_text(&self, priority: i32, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed && todo.priority.value == priority && todo.text.contains(text)).collect()
    }

    pub fn filter_tag_priority_text(&self, tag: &str, priority: i32, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.tag == tag && todo.priority.value == priority && todo.text.contains(text)).collect()
    }

    pub fn filter_completed_tag_text(&self, tag: &str, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed && todo.tag == tag && todo.text.contains(text)).collect()
    }

    pub fn filter_incomplete_tag_text(&self, tag: &str, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed && todo.tag == tag && todo.text.contains(text)).collect()
    }

    pub fn filter_priority_tag_text(&self, priority: i32, tag: &str, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.priority.value == priority && todo.tag == tag && todo.text.contains(text)).collect()
    }

    pub fn filter_completed_priority_tag_text(&self, priority: i32, tag: &str, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed && todo.priority.value == priority && todo.tag == tag && todo.text.contains(text)).collect()
    }

    pub fn filter_incomplete_priority_tag_text(&self, priority: i32, tag: &str, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed && todo.priority.value == priority && todo.tag == tag && todo.text.contains(text)).collect()
    }

    pub fn filter_completed_tag_priority_text(&self, tag: &str, priority: i32, text: &str) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed && todo.tag == tag && todo.priority.value == priority && todo.text.contains(text)).collect()
    }
}
