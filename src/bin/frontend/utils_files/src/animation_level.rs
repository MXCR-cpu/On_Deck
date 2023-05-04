use std::convert::From;

#[derive(Clone, PartialEq)]
pub enum AnimationLevel {
    None,
    Low,
    High,
}

pub trait ToStringify {
    fn convert_to_string(&self) -> String;
}

impl ToStringify for AnimationLevel {
    fn convert_to_string(&self) -> String {
        match self {
            AnimationLevel::High => "High",
            AnimationLevel::Low => "Low",
            AnimationLevel::None => "None",
        }.to_string()
    }
}

pub trait FromStringify {
    fn convert_from_string(&self) -> AnimationLevel;
}

impl FromStringify for String {
    fn convert_from_string(&self) -> AnimationLevel {
        match self.as_str() {
            "High" => AnimationLevel::High,
            "Low" => AnimationLevel::Low,
            "None" => AnimationLevel::None,
            _ => AnimationLevel::None,
        }
    }
}

impl From<AnimationLevel> for usize {
    fn from(item: AnimationLevel) -> Self {
        match item.clone() {
            AnimationLevel::High => 2,
            AnimationLevel::Low => 1,
            AnimationLevel::None => 0,
        }
    }
}
