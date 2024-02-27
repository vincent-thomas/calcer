// Hjälpstruct för att visa att bara plus och minus är tillåtet som operationer mellan termer, denna seperation mellan plus och minus görs på grund av prioriteringsreglerna.
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Plus,
    Minus,
}

// Trait som implementeras för att kunna parsea en sträng till en Operation
impl From<&str> for Operation {
    // Funktionen som implementeras från traiten, tar in en &str och returnerar en Operation
    fn from(raw: &str) -> Self {
        match raw {
            "+" => Self::Plus,
            "-" => Self::Minus,
            _ => panic!("Invalid operation"),
        }
    }
}

// Hjälpfunktion för att applicera en operation på två f64
impl Operation {
    pub fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Self::Plus => a + b,
            Self::Minus => a - b,
        }
    }
}
