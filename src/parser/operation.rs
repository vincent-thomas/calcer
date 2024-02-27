// Hjälpstruct för att visa att bara divition och multiplikation är tillåtet i en term, denna seperation mellan plus, minus och gånger och divition görs på grund av prioriteringsreglerna.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operation {
    Div,
    Mult,
}

// Trait som implementeras för att kunna parsea en sträng till en Operation
impl From<&str> for Operation {
    // Funktionen som implementeras från traiten, tar in en &str och returnerar en Operation
    fn from(raw: &str) -> Self {
        match raw {
            "/" => Self::Div,
            "*" => Self::Mult,
            // Invalid operation om det inte är "/" eller "*",
            // panic! är ungefär som att throwa en error i andra språk, bara att man inte kan "catcha" de i rust
            _ => panic!("Invalid operation"),
        }
    }
}

// Hjälpfunktion för att applicera en operation på två f64
impl Operation {
    pub fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Self::Div => a / b,
            Self::Mult => a * b,
        }
    }
}
