use crate::parser::TermParser;

use super::operation::Operation;

// Hjälpfunktion som omvandlar en vektor av charactärer i ett matteproblem, och parsar det till en vektor av TermParser och Option<Operation>
// En Option är en enum som antingen är Some(värde) eller None
// Den är None när TermParser är det sista talet i ett problem
pub fn transform_raw_to_numbers(raw: &[&str]) -> Vec<(TermParser, Option<Operation>)> {
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
