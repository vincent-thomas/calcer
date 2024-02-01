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

    pub fn write_with_diagram(&self, start: i64, end: i64) {
        let window_x = end - start + 1;

        dbg!(window_x);

        let mut graph = Graph::new(start, end, self.clone(), 10.0);

        let origo = graph.set_origo(start, end);

        graph.matrix.iter().for_each(|v| {
            println!("{:?}", v);
        });
    }
}

#[derive(Debug, Clone)]
enum CordinateValue {
    Empty,
    Value(Cordinate),
    Origo,
}

#[derive(Debug, Clone)]
struct Cordinate(usize, usize);

impl From<&str> for Problem {
    fn from(raw_input: &str) -> Self {
        let str_vec = raw_input.split("").collect::<Vec<&str>>()[1..].to_vec();

        let parsed_input = transform_raw_to_numbers(&str_vec);
        Self(parsed_input)
    }
}

struct Graph {
    pub matrix: Vec<Vec<CordinateValue>>,
    origo: Option<Cordinate>,
    problem: Problem,
}

impl Graph {
    pub fn new(def_start: i64, def_end: i64, problem: Problem, incr: f64) -> Self {
        let max_value = problem.clone().solve(Some(def_end as f64));
        let min_value = problem.clone().solve(Some(def_start as f64));
        dbg!(min_value, max_value);
        let minus_origo_y = (min_value / incr).round().abs();
        let plus_origo_y = (max_value / incr).ceil().abs();

        dbg!(plus_origo_y, minus_origo_y);

        Self {
            origo: None,
            matrix: vec![
                vec![CordinateValue::Empty; (def_end - def_start + 1) as usize];
                minus_origo_y as usize + plus_origo_y as usize + 1
            ],
            problem,
        }
    }

    pub fn set_origo(&mut self, def_start: i64, def_end: i64) -> Option<Cordinate> {
        dbg!(def_start, def_end);

        self.matrix = self
            .matrix
            .iter()
            .enumerate()
            .map(|(y_index, value)| {
                value
                    .iter()
                    .enumerate()
                    .map(
                        |(x_index, _)| {
                            // dbg!(x_index, y_index);
                            match def_start < 0 {
                                true => {
                                    if
                                    // self.matrix.len() as i64 - y_index as i64 - 1 == -def_start
                                    // &&
                                    x_index as i64 == -def_start {
                                        self.origo = Some(Cordinate(x_index, y_index));
                                        CordinateValue::Origo
                                    } else {
                                        CordinateValue::Empty
                                    }
                                }
                                false => CordinateValue::Empty,
                            }
                        }, // match def_start >= 0 {
                           //     false => {
                           //         dbg!(x_index, y_index);
                           //         if 10 - y_index as i64 - 1 == def_start
                           //             && x_index as i64 == tmp_origin_x
                           //         {
                           //             self.origo = Some(Cordinate(x_index, y_index));
                           //             CordinateValue::Origo
                           //         } else {
                           //             CordinateValue::Empty
                           //         }
                           //     }
                           //     true => {
                           //         dbg!(x_index, y_index);
                           //         if y_index as i64 == tmp_origin_x && x_index == 0 {
                           //             self.origo = Some(Cordinate(tmp_origin_x as usize, 0));
                           //             CordinateValue::Origo
                           //         } else {
                           //             CordinateValue::Empty
                           //         }
                           //     }
                           // }
                    )
                    .collect()
            })
            .collect::<Vec<Vec<CordinateValue>>>();
        self.origo.clone()
    }
}
