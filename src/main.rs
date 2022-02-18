use std::env;
use solver;

fn usage(program: &str) {
    println!(
        "Usage: to solve a polynomial of degree 2 or less
    ./{} \"5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0\"",
        program
    );
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    match args.len() {
        2 => {
            let input: String = args[1].to_string();
            solver::solve(input)?;
            Ok(())
        }
        _ => {
            usage(&program);
            Err("program need ONE argument!")
        }
    }
}
