use std::env::args;

use calc::problem::Problem;
use clier_parser::Argv;

fn main() {
    let args: Vec<String> = args().collect();

    let args = Argv::from(args.as_slice());

    // let commands = &args.commands[1..];

    // dbg!(commands);

    // dbg!(args.flags);

    let problem_raw: &str = args.flags.get("problem").unwrap();
    let raw_x = args.flags.get("range");

    let problem = Problem::from(problem_raw);
    match raw_x {
        Some(_tal) => {
            let raw_tal = _tal.split("..").collect::<Vec<&str>>();
            let start: i64 = raw_tal[0].parse().expect("Not a number");
            let end = raw_tal[1].parse::<i64>().expect("Not a number");

            problem.write_with_diagram(start, end);
        }
        None => {
            println!("y = {}", problem.clone().solve(None))
        }
    };
}
