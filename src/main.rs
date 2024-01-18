use std::env::args;

use calc::{
    parser::TermParser,
    term::{MathOperations, Operation, Term, Value},
};
use clier_parser::Argv;

fn main() {
    let args: Vec<String> = args().collect();

    let test = Argv::from(args.as_slice());

    let parser = TermParser::from("5*5/5");
    parser.parse();
    // let problem = Term::new(
    //     Value::Number(8.2).sqrt(),
    //     vec![(
    //         Operation::Div,
    //         Value::Parantes(Box::new(Term::new(
    //             Value::Number(2.0),
    //             vec![(Operation::Plus, Value::Number(3.4))],
    //         )))
    //         .sqrt(),
    //     )],
    // );
    // println!("{}", problem.solve());
}
