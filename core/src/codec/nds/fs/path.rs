use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NdsPath(String);

impl Default for NdsPath {
    fn default() -> Self {
        Self::new()
    }
}

impl NdsPath {
    pub fn new() -> Self {
        NdsPath(String::new())
    }

    pub fn push(&mut self, component: &str) {
        if !self.0.is_empty() && !self.0.ends_with('/') {
            self.0.push('/');
        }
        self.0.push_str(component);
    }

    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.0.split('/').filter(|s| !s.is_empty())
    }

    pub fn is_empty(&self) -> bool {
        self.components().count() == 0
    }
}

impl From<&str> for NdsPath {
    fn from(s: &str) -> Self {
        NdsPath(s.to_string())
    }
}

impl From<String> for NdsPath {
    fn from(s: String) -> Self {
        NdsPath(s)
    }
}

impl fmt::Display for NdsPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
