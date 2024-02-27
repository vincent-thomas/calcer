use std::env::args;

use calcer::problem::Problem;
use clier_parser::Argv;

fn main() {
    // Råa args from terminalen
    let args: Vec<String> = args().collect();

    // Parsar argsen till en Argv struct, (mitt externa library https://docs.rs/clier_parser)
    let args = Argv::from(args.as_slice());

    // Tar bort första argumentet som är programnamnets exe
    let commands = &args.commands[1..];

    // Tar ut en flagga här
    let problem = args.flags.get("problem").expect("Missing problem");
    // Problem::from() tar en pointer till en statisk sträng och gör det till ett Problem struct
    // Den parsar matteproblemet och gör så att det kan lösas
    let problem = Problem::from(problem.as_str());

    // 'match' är som en switch statement i många språk fast i rust MÅSTE alla möjligheter av värdet hanteras
    match commands
        .first()
        .unwrap_or(&"NOT_A_COMMAND".to_string()) // Unwrap or är en metod som antingen returnerar värdet om det finns eller en default som i detta fall är "NOT_A_COMMAND"
        .as_str()
    {
        "solve" => {
            // Bara ett matteproblem inte något okänt värde, därför None
            println!("{}", problem.clone().solve(None))
        }
        "graph" => {
            let raw_x = match args.flags.get("definition") {
                Some(range) => range,
                None => {
                    println!("Missing range");
                    std::process::exit(1);
                }
            };

            // Första elemented är start, och andra (sista) är end
            let raw_tal: Vec<&str> = raw_x.split("..").collect();
            let start: i64 = raw_tal[0].parse().expect("Not a number");
            let end: i64 = raw_tal[1].parse().expect("Not a number");

            // Helper funktion som skriver ut en graf med problemet
            problem.write_with_diagram(start, end);
        }
        // Kommand som inte finns
        _ => {
            println!("Unknown command");
            std::process::exit(1);
        }
    }
}
