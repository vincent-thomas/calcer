use std::env::args;

use calc::problem::Problem;
use clier_parser::Argv;

fn main() {
    let args: Vec<String> = args().collect();

    let args = Argv::from(args.as_slice());

    let problem_raw: &str = args.flags.get("problem").unwrap();
    let raw_x = args.flags.get("range");

    let problem = Problem::from(problem_raw);
    let x = match raw_x {
        Some(tal) => {
            let raw_tal = tal.split("..").collect::<Vec<&str>>();
            let start: i64 = raw_tal[0].parse().expect("Not a number");
            let end = raw_tal[1].parse::<i64>().expect("Not a number");

            let mut array = vec![];
            let mut index = start;
            loop {
                if index > end {
                    break;
                };
                array.push(index);
                index += 1;
            }

            Some(array)
        }
        None => None,
    };

    // dbg!(&problem);

    x.unwrap().iter().for_each(|x| {
        println!("y({}) = {}", x, problem.clone().solve(Some(*x as f64)));
    });
}
