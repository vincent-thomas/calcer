#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operation {
    Div,
    Mult,
}

impl From<&str> for Operation {
    fn from(raw: &str) -> Self {
        match raw {
            "/" => Self::Div,
            "*" => Self::Mult,
            _ => panic!("Invalid operation"),
        }
    }
}

impl Operation {
    pub fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Self::Div => a / b,
            Self::Mult => a * b,
        }
    }
}
