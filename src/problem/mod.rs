use crate::{graph::Graph, parser::TermParser};

// "Problem" används för att hantera ett helt problem.
// Ett "Problem" bestär av en vektor av "TermParser" (termer) och en vektor av "Option<Operation>" (operationer)
mod problem_parser;
// "Operation" används för att hantera operationer som addition och subtraktion
use operation::Operation;

use self::problem_parser::transform_raw_to_numbers;
mod operation;

#[derive(Debug, Clone)]
pub struct Problem(pub Vec<(TermParser, Option<Operation>)>);

// Funktioner till problem, ungefär som methods/static methods till en klass i andra språk
impl Problem {
    // Tar in Problem och en Option<f64> och löser hela problemet som finns innuti Problemstructen -> f64, Option<f64> är till om det finns en eventuellt x-värde som ska bytas ut
    // Den gör detta genom att iterera över alla termer och operationer och applicera operationen på termerna
    pub fn solve(self, unknown: Option<f64>) -> f64 {
        let parsed_input = &self.0;
        let first_value = parsed_input.first().expect("No first value"); // .expect() returnerar OM Some(x) -> x, OM None -> panic med medelande man skriver till argument
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

    // Hjälpfunktion som tar in en pointer till Problem, och definitionsvärde (start, end) som är i form av i64, och aktiverar funktioner från "Graph", så att en graph modelleras och ritas.
    pub fn write_with_diagram(&self, start: i64, end: i64) {
        let mut graph = Graph::new(start, end, self);
        graph.set_origo();
        graph.write();
        graph.average();
        graph.median();
        graph.graph();
    }
}

// From traiten är en trait som låter oss konvertera en typ till en annan
// I detta fall konverterar vi en pointer till en statisk sträng (&str) till ett Problem struct
impl From<&str> for Problem {
    fn from(raw_input: &str) -> Self {
        let str_vec = raw_input.split("").collect::<Vec<&str>>()[1..].to_vec();
        Self(transform_raw_to_numbers(&str_vec))
    }
}
