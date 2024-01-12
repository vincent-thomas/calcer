#[derive(Debug, Clone)]
pub enum Operation {
    Div,
    Mult,
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Parantes(Box<Term>),
}

impl MathOperations for Value {
    fn pow(&self, power: i32) -> Self {
        match self.clone() {
            Value::Number(number) => Value::Number(number.powi(power)),
            Value::Parantes(number) => Self::Parantes(Box::new(Term {
                value: number.value,
                operations: number.operations,
                should_sqrt: false,
                power_to: power,
            })),
        }
    }
    fn sqrt(&self) -> Self {
        match self.clone() {
            Value::Number(number) => Value::Number(number.sqrt()),
            Value::Parantes(number) => Self::Parantes(Box::new(Term {
                value: number.value,
                operations: number.operations,
                should_sqrt: true,
                power_to: 1,
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Term {
    value: Value,
    operations: Vec<(Operation, Value)>,
    power_to: i32,
    should_sqrt: bool,
}

impl Term {
    pub fn new(value: Value, operations: Vec<(Operation, Value)>) -> Self {
        Self {
            power_to: 1,
            should_sqrt: false,
            value,
            operations,
        }
    }
    pub fn solve(&self) -> f32 {
        let mut total = match self.value.clone() {
            Value::Number(number) => number,
            Value::Parantes(term) => (*term).solve(),
        };

        self.clone()
            .operations
            .iter()
            .map(|(operation, value)| match value.clone() {
                Value::Number(number) => (operation, number),
                Value::Parantes(param) => (operation, param.solve()),
            })
            .for_each(|value| match value.0.clone() {
                Operation::Plus => total += value.1,
                Operation::Minus => total -= value.1,
                Operation::Mult => total *= value.1,
                Operation::Div => total /= value.1,
            });

        if self.should_sqrt {
            total.sqrt()
        } else {
            total.powi(self.power_to)
        }
    }
}

pub trait MathOperations {
    fn pow(&self, power: i32) -> Self;

    fn sqrt(&self) -> Self;
}
