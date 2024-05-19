use calcer::problem::Problem;
use clier::{
    display::{label::LabelLogger, Displayer},
    hooks::{use_double_dash, use_flag, FlagError},
    run::ExitCode,
    CliMeta, Clier, CmdMeta, Commands,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn meta() -> CliMeta {
    CliMeta::new(
        "clier".to_string(),
        "Clier is a nice math thing".to_string(),
    )
    .version(VERSION)
    .usage("[command]")
}

fn solve_command() -> Commands {
    Commands::Command {
        meta: CmdMeta::new("solve", "only do problems"),
        handler: |clier| {
            let log = LabelLogger::default();
            let problem_str = match use_double_dash(&clier) {
                Ok(value) => value,
                Err(_) => {
                    log.error("Problem is required and should be defined after '--'");
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
            let log = LabelLogger::default();

            let flag_result: Result<String, FlagError> =
                use_flag("definition", Some('d'), &clier).try_into();

            let flag = match flag_result {
                Ok(value) => value,
                Err(_) => {
                    log.error("Missing range");
                    return ExitCode(1);
                }
            };
            // Första elemented är start, och andra (sista) är end
            let raw_tal: Vec<&str> = flag.split("..").collect();
            let start: i64 = match raw_tal[0].parse() {
                Ok(value) => value,
                Err(_) => {
                    log.error("Failed to parse start range");
                    return ExitCode(1);
                }
            };
            let end: i64 = match raw_tal[1].parse() {
                Ok(value) => value,
                Err(_) => {
                    log.error("Failed to parse end range");
                    return ExitCode(1);
                }
            };

            let problem_str = match use_double_dash(&clier) {
                Ok(value) => value,
                Err(_) => {
                    log.error("Problem is not defined");
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
