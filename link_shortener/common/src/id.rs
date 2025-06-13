use crate::generators::generate_id;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Self(generate_id())
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(value)
    }
}
