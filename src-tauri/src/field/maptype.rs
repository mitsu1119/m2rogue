use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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
