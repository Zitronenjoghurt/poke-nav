use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Platform {
    Nds,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nds => write!(f, "Nintendo DS"),
        }
    }
}
