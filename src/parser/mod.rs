use crate::term::{Operation, Operations, Term, Value};

#[derive(Debug)]
pub struct TermParser {
    raw_input: String,
}

fn get_first_number(raw: &str) -> (i32, String) {
    let mut first_item_index = 0;
    let mut string_number = String::new();

    loop {
        let character = raw.chars().nth(first_item_index).unwrap();

        if character == '-' || character == '+' || character == '*' || character == '/' {
            break;
        }

        string_number.push_str(character.to_string().as_str());
        first_item_index += 1;
    }

    (
        string_number.parse::<i32>().unwrap(),
        raw[first_item_index..].to_string(),
    )
}

fn fetch_operations(str: &str) -> Operations {
    let mut str_vec = str.split("").collect::<Vec<&str>>()[1..].to_vec();
    str_vec.pop();

    let mut operations: Operations = Vec::new();

    dbg!(&str_vec);

    str_vec.windows(2).for_each(|window| {
        dbg!(window);
        operations.push((
            Operation::from(window[0]),
            Value::Number(window[1].parse().unwrap()),
        ));
    });

    dbg!(operations);

    todo!()
}

impl TermParser {
    pub fn from(raw_input: &str) -> Self {
        Self {
            raw_input: raw_input.into(),
        }
    }
    pub fn parse(&self) {
        dbg!(self);

        // let mut parsed_round_1 = Vec::new();

        // let mut parantes_when_parsing = 0;

        let (first_number, input) = get_first_number(&self.raw_input);
        dbg!(&input);
        let operations = fetch_operations(&input);

        // while index < self.raw_input.len() {
        //     let character = self.raw_input.chars().nth(index).unwrap();

        //     let mut parantes_when_parsing = 0;
        //     if character == '(' {
        //         let mut char_index = index.clone();
        //         let mut parantes = String::new();
        //         loop {
        //             let character = self.raw_input.chars().nth(char_index).unwrap();
        //             if character == ')' {
        //                 parantes_when_parsing -= 1;
        //             }
        //             if character == '(' {
        //                 parantes_when_parsing += 1;
        //             }
        //             if character == ')' && parantes_when_parsing == 0 {
        //                 char_index += 1;
        //                 parantes.push_str(
        //                     format!("{}", &self.raw_input[index..char_index])
        //                         .as_str()
        //                         .as_ref(),
        //                 );
        //                 break;
        //             }
        //             char_index += 1;
        //         }
        //         dbg!(parantes);
        //     }

        // if character == '(' {
        //     parantes_when_parsing += 1;
        //     continue;
        // }
        // if character == ')' {
        //     parantes_when_parsing -= 1;
        //     continue;
        // }
        // println!(" test {} {}", character, parantes_when_parsing);
        // if parantes_when_parsing != 0 {
        //     let index_param_start = index;

        //     let mut index_in_param = index;
        //     let mut param = String::new();
        //     let mut extra_params: i32 = 0;
        //     loop {
        //         println!("{} {}", character, index_in_param);
        //         if character == '(' {
        //             index_in_param += 1;
        //             extra_params += 1;
        //             continue;
        //         }
        //         if character == ')' && extra_params == 0 {
        //             param.push_str(
        //                 format!("{}", &self.raw_input[index_param_start..index_in_param])
        //                     .as_str()
        //                     .as_ref(),
        //             );
        //             dbg!(param);
        //             break;
        //         }
        //         if character == ')' {
        //             index_in_param -= 1;
        //             extra_params -= 1;
        //             continue;
        //         }
        //         // }
        //     }
        //     println!("{}", character);

        //     parsed_round_1.push(character);

        //     index += 1;
        // }

        // let output = self
        //     .raw_input
        //     .chars()
        //     .enumerate()
        //     .filter(|(index, c)| -> bool {
        //         if c == &'(' {
        //             parantes_when_parsing += 1;
        //         }
        //         if c == &')' {
        //             parantes_when_parsing -= 1;
        //         }
        //         if (c == &'+' || c == &'-') && parantes_when_parsing == 0 {
        //             let slice = &self.raw_input[last_split_index..index.clone()];
        //             parsed_round_1.push(slice);
        //             last_split_index = self.raw_input.find(c.clone()).unwrap();
        //             false
        //         } else {
        //             true
        //         }
        //     });

        // println!("{:?}", parsed_round_1);

        // index += 2;
    }
}
