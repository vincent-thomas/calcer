use std::env::args;

use calc::parser::TermParser;
use clier_parser::Argv;

fn main() {
    let args: Vec<String> = args().collect();

    let args = Argv::from(args.as_slice());

    let skit = args.flags.get("parse").unwrap();

    let parser = TermParser::from(skit).parse();

    println!("{}", parser.solve());
}
