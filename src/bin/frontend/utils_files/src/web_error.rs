use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug)]
pub struct ClientError {
    pub stack: Vec<String>,
}

impl ClientError {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn from(current_file: &str, new_error: &str) -> Self {
        let mut stack = Vec::new();
        let current_file_path_buf: String = Self::shorten_path(current_file);
        stack.push(format!("{}: {}", current_file_path_buf, new_error));
        Self { stack }
    }

    pub fn push(&self, current_file: &str, new_error: &str) -> Self {
        let mut stack: Vec<String> = self.stack.clone();
        let current_file_path_buf: String = Self::shorten_path(current_file);
        stack.push(format!("{}: {}", current_file_path_buf, new_error));
        Self { stack }
    }

    fn shorten_path(file_path: &str) -> String {
        PathBuf::from(file_path)
            .iter()
            .rev()
            .enumerate()
            .filter(|(index, _): &(usize, &OsStr)| index < &3)
            .collect::<Vec<(usize, &OsStr)>>()
            .iter()
            .rev()
            .map(|(_, value): &(usize, &OsStr)| value.to_str().unwrap().to_owned())
            .collect::<Vec<String>>()
            .join("/")
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

