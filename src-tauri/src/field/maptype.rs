use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapType {
    Wall,
    Road
}

impl fmt::Display for MapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Wall => write!(f, "🟧"),
            Self::Road => write!(f, "・")
        }
    }
}
