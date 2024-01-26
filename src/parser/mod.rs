use crate::term::Operation;

#[derive(Debug, Clone)]
pub struct TermParser {
    raw_input: String,
    pub parsed_input: Option<Vec<(f64, Option<Operation>)>>,
}

enum Modifier {
    None,
    RotenUr(f64),
}

struct Number {
    value: f64,
    operation: Modifier,
}

const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn parse_to_number(raw_num: &str) -> Number {
    // aNVÄND NUMBERS FÖR ATT FÖRSTÅ
    todo!()
}

fn transform_raw_to_numbers(raw: &[&str]) -> Vec<(f64, Option<Operation>)> {
    let mut numbers = Vec::new();

    let mut index = 0;
    let mut before_index = 0;
    loop {
        if index == raw.len() - 1 {
            let number = raw[before_index..index + 1]
                .join("")
                .parse()
                .expect("Not a number");
            numbers.push((number, None));
            break;
        }
        if raw[index] == "*" || raw[index] == "/" {
            let raw_tal = raw[before_index..index].join("");
            let tal: f64 = raw_tal.parse().expect("Not a tal");

            let operation = Operation::from(raw[index]);
            numbers.push((tal, Some(operation)));

            before_index = index + 1;
        }

        index += 1;
    }
    numbers
}

impl TermParser {
    pub fn from(raw_input: &str) -> Self {
        Self {
            raw_input: raw_input.into(),
            parsed_input: None,
        }
    }
    pub fn parse(mut self) -> Self {
        let mut str_vec = self.raw_input.split("").collect::<Vec<&str>>()[1..].to_vec();

        str_vec.pop();

        self.parsed_input = Some(transform_raw_to_numbers(&str_vec));
        self
    }

    pub fn solve(self) -> f64 {
        let parsed_input = self.parsed_input.clone().expect("");
        let first_value = parsed_input.first().expect("No first value").clone();
        let mut initial_value = first_value.0;

        self.parsed_input
            .unwrap()
            .as_slice()
            .iter()
            .enumerate()
            .filter(|index: &(usize, &(f64, Option<Operation>))| index.0 != 0)
            .for_each(|(index, (value, operation))| {
                initial_value = parsed_input[index - 1]
                    .1
                    .unwrap()
                    .apply(initial_value, *value);
            });
        initial_value
    }
}
