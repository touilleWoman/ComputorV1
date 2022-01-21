use float_pretty_print::PrettyPrintFloat;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fmt;


struct Unit {
    a: f64,
    b: f64,
    c: f64,
    other: u64,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expression: String = vec![(self.c, "* X^0"), (self.b, "* X^1"), (self.a, "* X^2")]
            .into_iter()
            .map(|(x, y)| {
                if x == 0.0 {
                    "".to_string()
                } else if x > 0.0 {
                    format!("+ {} {}", x.to_string(), y)
                } else {
                    format!("- {} {}", (-x).to_string(), y)
                }
            })
            .collect();
        write!(f, "{} = 0", expression.trim_start_matches("+ "),)
    }
}

fn reduce(expression: String) -> Result<Unit, &'static str> {
    let (left, right) = match expression.split_once("=") {
        None => {
            panic!("Wrong input: No '=' in expression")
        }
        Some((x, y)) => (x.trim(), y.trim()),
    };
    let left_unit = extract_coefficients(left)?;
    let right_unit = extract_coefficients(right)?;
    let reduced_unit = Unit {
        a: left_unit.a - right_unit.a,
        b: left_unit.b - right_unit.b,
        c: left_unit.c - right_unit.c,
        other: left_unit.other - right_unit.other,

    };
    let degree = {
        if reduced_unit.other != 0 {
            reduced_unit.other
        }else if reduced_unit.a != 0.0 {
            2
        } else if reduced_unit.b != 0.0 {
            1
        } else {
            0
        }
    };

    println!(
        "Reduced form: {}\nPolynomial degree: {}",
        reduced_unit, degree
    );
    if degree > 2 {
        return Err("The polynomial degree is stricly greater than 2, I can't solve.");
    }
    Ok(reduced_unit)
}

// fn get_float(caps: &Captures, name: &str) -> f64 {
//     caps.name(name).map_or(0.0, |m| m.as_str().parse::<f64>().unwrap())
// }

fn extract_coefficients(expression: &str) -> Result<Unit, &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
            ^
            ((?P<c>[-]?\d+\.?\d*)[*]X\^0)?
            ((?P<b>[+-]?\d+\.?\d*)[*]X\^1)?
            ((?P<a>[+-]?\d+\.?\d*)[*]X\^2)?
            ([+-]?\d+\.?\d*[*]X\^(?P<other>\d+))?
            $
            "#
        )
        .unwrap();
    }
    match RE.captures(expression) {
        None => Err("wrong expression, nothing captured"),
        Some(caps) => {
            let unit = Unit {
                c: caps.name("c").map_or(0.0, |m| m.as_str().parse::<f64>().unwrap()),
                b: caps.name("b").map_or(0.0, |m| m.as_str().parse::<f64>().unwrap()),
                a: caps.name("a").map_or(0.0, |m| m.as_str().parse::<f64>().unwrap()),
                other: caps.name("other").map_or(0, |m| m.as_str().parse::<u64>().unwrap()),
            };
            Ok(unit)
        }
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace())
}

fn solve(data: Unit) {
    if data.a == 0.0 {
        println!(
            "The solution is:\n{:1.8}",
            PrettyPrintFloat(-data.c / data.b)
        );
        return;
    }

    let delta = data.b * data.b - 4.0 * data.a * data.c;
    if delta < 0.0 {
        println!("Discriminant is negative, there is no solution.");
    } else if delta == 0.0 {
        let solution = -data.b / (2.0 * data.a);
        println!(
            "Discriminant is zero, the one solution is:\n{:1.8}",
            PrettyPrintFloat(solution)
        );
    } else {
        let solution1 = (-data.b + delta.sqrt()) / (2.0 * data.a);
        let solution2 = (-data.b - delta.sqrt()) / (2.0 * data.a);
        println!(
            "Discriminant is strictly positive, the two solutions are:\n{:1.8}\n{:1.8}",
            PrettyPrintFloat(solution1),
            PrettyPrintFloat(solution2),
        );
    }
}

fn usage(program: &str) {
    println!(
        "Usage: to solve a polynomial of degree 2 or less
    ./{} \"5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0\"",
        program
    );
}

fn main()-> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    match args.len() {
        2 => {
            let mut input: String = args[1].to_string();
            remove_whitespace(&mut input);
            let data = reduce(input)?;
            solve(data);
            Ok(())
        }
        _ => {
            usage(&program);
            Err("program need ONE argument!")
        }
    }
}
