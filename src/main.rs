use calc::{MathOperations, Operation, Term, Value};

fn main() {
    let mut float_vector: Vec<f32> = Vec::new();

    for i in -10..=10 {
        let value = i as f32;
        float_vector.push(value);
    }

    // Function: y=x^2+2x+x in -10 =< x =< 10
    float_vector.iter().for_each(|value| {
        let problem = Term::new(
            Value::Number(*value).pow(2),
            vec![(
                Operation::Plus,
                Value::Parantes(Box::new(Term::new(
                    Value::Number(*value * 2.0),
                    vec![(Operation::Plus, Value::Number(*value))],
                ))),
            )],
        );
        println!("{} {}", value, problem.solve());
    })
}
