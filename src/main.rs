use std::env::args;

use calcer::problem::Problem;
use clier_parser::Argv;

fn main() {
    let args: Vec<String> = args().collect();

    let args = Argv::from(args.as_slice());

    let commands = &args.commands[1..];

    match commands
        .first()
        .unwrap_or(&"NOT_A_COMMAND".to_string())
        .as_str()
    {
        "solve" => {
            let problem_raw: &str = args.flags.get("problem").expect("Missing problem");
            let problem = Problem::from(problem_raw);

            println!("{}", problem.clone().solve(None))
        }
        "graph" => {
            let problem_raw: &str = args.flags.get("problem").unwrap();
            let raw_x = match args.flags.get("range") {
                Some(range) => range,
                None => {
                    println!("Missing range");
                    std::process::exit(1);
                }
            };

            let problem = Problem::from(problem_raw);
            let raw_tal = raw_x.split("..").collect::<Vec<&str>>();
            let start: i64 = raw_tal[0].parse().expect("Not a number");
            let end = raw_tal[1].parse::<i64>().expect("Not a number");

            problem.write_with_diagram(start, end);
        }
        _ => {
            println!("Unknown command");
            std::process::exit(1);
        }
    }
}
