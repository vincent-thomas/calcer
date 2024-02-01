mod operation;

use operation::Operation;

#[derive(Debug, Clone, Copy)]
pub struct Number {
    value: Int,
    power_to: Int,
}

impl From<&str> for Number {
    fn from(raw_number: &str) -> Self {
        let can_be_power_to = raw_number.contains('^');

        let can_be_square = raw_number.starts_with("sqrt(") && raw_number.ends_with(')');

        let number: Int;

        let mut power_to = Int::Value(1.0);
        if can_be_power_to {
            let index = raw_number.find('^').unwrap();
            number = Int::from(&raw_number[..index]);
            power_to = Int::from(&raw_number[index + 1..]);
        } else if can_be_square {
            let raw_number = &raw_number[5..raw_number.len() - 1];
            number = Int::from(raw_number);
            power_to = Int::Value(0.5);
        } else {
            number = Int::from(raw_number)
        }

        Self {
            value: number,
            power_to,
        }
    }
}

impl Number {
    pub fn solve(&self, to_replace_unknown: Option<f64>) -> f64 {
        match (self.value, self.power_to) {
            (Int::Value(value), Int::Value(pow)) => value.powf(pow),
            (Int::Value(value), Int::Unknown) => {
                value.powf(to_replace_unknown.expect("No value to use for x"))
            }
            (Int::Unknown, Int::Value(pow)) => {
                to_replace_unknown.expect("No value to use for x").powf(pow)
            }
            (Int::Unknown, Int::Unknown) => to_replace_unknown
                .expect("No value to replace unknown")
                .powf(to_replace_unknown.expect("No value to use for x")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Int {
    Value(f64),
    Unknown,
}

impl From<&str> for Int {
    fn from(value: &str) -> Self {
        if value == "x" {
            Self::Unknown
        } else {
            Self::Value(value.parse::<f64>().unwrap())
        }
    }
}

#[derive(Debug, Clone)]
pub struct TermParser(pub Vec<(Number, Option<Operation>)>);

fn transform_raw_to_numbers(raw: &[&str]) -> Vec<(Number, Option<Operation>)> {
    let mut numbers = Vec::new();

    let mut index = 0;
    let mut before_index = 0;
    loop {
        if index == raw.len() - 1 {
            let raw_number = raw[before_index..index + 1].join("");

            let number: Number = Number::from(raw_number.as_str());
            numbers.push((number, None));
            break;
        }
        if raw[index] == "*" || raw[index] == "/" {
            let raw_tal = raw[before_index..index].join("");

            let tal = Number::from(raw_tal.as_str());

            let operation = Operation::from(raw[index]);
            numbers.push((tal, Some(operation)));

            before_index = index + 1;
        }
        index += 1;
    }

    numbers
}

impl TermParser {
    pub fn solve(self, to_replace_unknown: Option<f64>) -> f64 {
        let parsed_input = self.0.clone();
        let first_value = parsed_input.first().expect("No first value");
        let mut initial_value = first_value.0.solve(to_replace_unknown);

        self.0
            .as_slice()
            .iter()
            .enumerate()
            .filter(|index: &(usize, &(Number, Option<Operation>))| index.0 != 0)
            .for_each(|(index, (value, _))| {
                let computable_value = value.solve(to_replace_unknown);

                initial_value = parsed_input[index - 1]
                    .1
                    .unwrap()
                    .apply(initial_value, computable_value);
            });
        initial_value
    }
}

impl From<&str> for TermParser {
    fn from(raw_input: &str) -> Self {
        let mut str_vec = raw_input.split("").collect::<Vec<&str>>()[1..].to_vec();
        str_vec.pop();
        Self(transform_raw_to_numbers(&str_vec))
    }
}
