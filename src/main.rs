use calcer::problem::Problem;
use clier::{
    display::Displayer,
    hooks::{use_double_dash, use_flag, FlagError},
    run::ExitCode,
    CliMeta, Clier, CmdMeta, Commands,
};

fn meta() -> CliMeta {
    CliMeta {
        name: "clier".to_string(),
        description: "Clier is a nice math thing".to_string(),
        version: Some((0, 1, 0)),
        usage: Some("[command]".to_string()),
    }
}

fn solve_command() -> Commands {
    Commands::Command {
        meta: CmdMeta::new("solve", "only do problems"),
        handler: |clier| {
            let error = Displayer::Error {};
            let problem_str = match use_double_dash(&clier) {
                Ok(value) => value,
                Err(_) => {
                    error.write("Problem is required and should be defined after '--'");
                    return ExitCode(1);
                }
            };
            let problem = Problem::from(problem_str.as_str());
            println!("{}", problem.solve(None));
            ExitCode(0)
        },
    }
}

fn graph_command() -> Commands {
    Commands::Command {
        meta: CmdMeta::new("graph", "Use for graphing math problem with 'x'"),
        handler: |clier| {
            let error = Displayer::Error {};

            let flag_result: Result<String, FlagError> =
                use_flag("definition", Some('d'), &clier).try_into();

            let flag = match flag_result {
                Ok(value) => value,
                Err(_) => {
                    error.write("Missing range");
                    return ExitCode(1);
                }
            };
            // Första elemented är start, och andra (sista) är end
            let raw_tal: Vec<&str> = flag.split("..").collect();
            let start: i64 = match raw_tal[0].parse() {
                Ok(value) => value,
                Err(_) => {
                    error.write("Failed to parse start range");
                    return ExitCode(1);
                }
            };
            let end: i64 = match raw_tal[1].parse() {
                Ok(value) => value,
                Err(_) => {
                    error.write("Failed to parse end range");
                    return ExitCode(1);
                }
            };

            let problem_str = match use_double_dash(&clier) {
                Ok(value) => value,
                Err(_) => {
                    error.write("Problem is not defined");
                    return ExitCode(1);
                }
            };

            let problem = Problem::from(problem_str.as_str());

            problem.write_with_diagram(start, end);
            ExitCode(0)
        },
    }
}

fn main() -> clier::run::ExitCode {
    let clier_builder = Clier::parse();
    let clier = clier_builder.meta(meta());
    let app = clier.runnable(vec![solve_command(), graph_command()]);
    app.run()
}
