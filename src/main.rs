#[derive(Debug, Clone)]
enum Operation {
    Div,
    Mult,
}

#[derive(Debug, Clone)]
enum Binder {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
struct Calculation {
    binder: Binder,
    term: Term,
    next: Option<Box<Calculation>>,
}

impl Calculation {
    fn solve(&self) {
        let left_value = self.term.to_value_number();
        let right_value = match self.next.clone() {
            Some(value) => value.term.to_value_number(),
            None => Value {
                base: 1,
                exponent: 1,
            },
        };
        dbg!(left_value, right_value);
    }
}

impl Term {
    fn to_value_number(&self) -> Value {
        match self.clone() {
            Term::Value(value) => value,
            Term::Uttryck(uttryck) => {
                let value = uttryck.value;
                let next_value = match uttryck.next {
                    Some(value) => value.to_value(),
                    None => Value {
                        base: 1,
                        exponent: 1,
                    },
                };
                Value {
                    base: next_value.base.pow(next_value.exponent) + value.base.pow(value.exponent),
                    exponent: 1,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Term {
    Value(Value),
    Uttryck(Uttryck),
}

#[derive(Debug, Clone)]
struct Uttryck {
    next_operation: Operation,
    value: Value,
    next: Option<Box<Uttryck>>,
}

impl Uttryck {
    fn to_value(&self) -> Value {
        let left = self.value.clone();
        let right = self
            .next
            .clone()
            .unwrap_or(Box::new(Uttryck {
                next: None,
                next_operation: Operation::Mult,
                value: Value {
                    base: 0,
                    exponent: 0,
                },
            }))
            .to_value();
        Value {
            exponent: 1,
            base: match self.next_operation {
                Operation::Div => left.to_number() / right.to_number(),
                Operation::Mult => left.to_number() * right.to_number(),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Value {
    base: i32,
    exponent: u32,
}

impl Value {
    fn to_number(&self) -> i32 {
        self.base.pow(self.exponent)
    }
}

fn main() {
    let problem = Calculation {
        binder: Binder::Plus,
        term: Term::Uttryck(Uttryck {
            value: Value {
                base: 4,
                exponent: 1,
            },
            next: None,
            next_operation: Operation::Mult,
        }),
        next: None,
    }
    .solve();
    // let matte_problem = Calculation {
    //     operation: Binders::Plus,
    //     value: Value::Annan(
    //         Box::new(Value::ValueNumber(ValueNumber {
    //             base: 6,
    //             exponent: 7,
    //         })),
    //         Operation::Mult,
    //         Box::new(Value::Annan(ValueNumber {
    //             base: 2,
    //             exponent: 5,
    //         })),
    //     ),
    //     next: None,
    // };
    // dbg!(matte_problem.value.to_value_number().to_number());
}
