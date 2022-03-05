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
