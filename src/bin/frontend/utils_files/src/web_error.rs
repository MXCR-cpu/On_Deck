use std::fmt;

#[derive(Clone)]
pub struct ClientError {
    stack: Vec<String>,
}

impl ClientError {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn from(current_file: &str, new_error: &str) -> Self {
        let mut stack = Vec::new();
        stack.push(format!("{}: {}", current_file, new_error));
        Self { stack }
    }

    pub fn push(&self, current_file: &str, new_error: &str) -> Self {
        let mut stack: Vec<String> = self.stack.clone();
        stack.push(format!("{}: {}", current_file, new_error));
        Self { stack }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut index: usize = 0;
        write!(
            f,
            "{}",
            self.stack
                .clone()
                .into_iter()
                .rev()
                .map(|element: String| {
                    index += 1;
                    format!("{}{}", "\t".repeat(index - 1), element)
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
