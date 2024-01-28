#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Plus,
    Minus,
}

impl From<&str> for Operation {
    fn from(raw: &str) -> Self {
        match raw {
            "+" => Self::Plus,
            "-" => Self::Minus,
            _ => panic!("Invalid operation"),
        }
    }
}

impl Operation {
    pub fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Self::Plus => a + b,
            Self::Minus => a - b,
        }
    }
}
