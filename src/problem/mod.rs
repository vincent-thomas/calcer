use crate::parser::TermParser;

use operation::Operation;
mod operation;

#[derive(Debug, Clone)]
pub struct Problem(pub Vec<(TermParser, Option<Operation>)>);

fn transform_raw_to_numbers(raw: &[&str]) -> Vec<(TermParser, Option<Operation>)> {
    let mut numbers = Vec::new();

    let mut index = 0;
    let mut before_index = 0;
    loop {
        if index == raw.len() - 1 {
            let raw_tal: &str = &raw[before_index..index].join("");

            let number = TermParser::from(raw_tal);
            numbers.push((number, None));
            break;
        }
        if raw[index] == "-" || raw[index] == "+" {
            let raw_tal: &str = &raw[before_index..index].join("");

            let operation = Operation::from(raw[index]);

            numbers.push((TermParser::from(raw_tal), Some(operation)));

            before_index = index + 1;
        }

        index += 1;
    }
    numbers
}

impl Problem {
    pub fn solve(self, unknown: Option<f64>) -> f64 {
        let parsed_input = &self.0;
        let first_value = parsed_input.first().expect("No first value");
        let mut initial_value = first_value.0.clone().solve(unknown);
        self.0
            .as_slice()
            .iter()
            .enumerate()
            .filter(|index: &(usize, &(TermParser, Option<Operation>))| index.0 != 0)
            .for_each(|(index, (value, _))| {
                let operation = self.0[index - 1].1;

                initial_value = match operation {
                    Some(operation) => operation.apply(initial_value, value.clone().solve(unknown)),
                    None => panic!("No operation"),
                };
            });

        initial_value
    }
}

impl From<&str> for Problem {
    fn from(raw_input: &str) -> Self {
        let str_vec = raw_input.split("").collect::<Vec<&str>>()[1..].to_vec();

        let parsed_input = transform_raw_to_numbers(&str_vec);
        Self(parsed_input)
    }
}
