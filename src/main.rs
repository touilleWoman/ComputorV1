use float_pretty_print::PrettyPrintFloat;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::env;

type Data = BTreeMap<u64, f64>;

fn print_reduced(reduced: &Data) {
    let mut expression: String = String::new();
    for (&key, &value) in reduced.iter() {
        if value.is_sign_negative() {
            expression.push_str(&format!("- {} * X^{} ", (-value).to_string(), key));
        } else {
            expression.push_str(&format!("+ {} * X^{} ", value.to_string(), key));
        }
    }
    println!("Reduced form: {} = 0", expression.trim_start_matches("+ "));
}

fn reduce(expression: String) -> Result<Data, &'static str> {
    let (left, right) = match expression.split_once("=") {
        None => {
            panic!("Wrong input: No '=' in expression")
        }
        Some((x, y)) => (x.trim(), y.trim()),
    };
    let mut left_data = extract_coefficients(left)?;
    let right_data = extract_coefficients(right)?;
    for (key, value) in right_data {
        if let Some(x) = left_data.get_mut(&key) {
            *x -= value;
        } else {
            left_data.insert(key, -value);
        }
    }
    let reduced = left_data;
    let degree = match reduced.keys().max() {
        None => return Err("No power in expression"),
        Some(x) => *x,
    };
    print_reduced(&reduced);
    println!("Polynomial degree: {}", degree);
    if degree > 2 {
        return Err("The polynomial degree is stricly greater than 2, I can't solve.");
    }
    Ok(reduced)
}

fn extract_coefficients(expression: &str) -> Result<Data, &'static str> {
    // first element may not have "+-"",  after element must have
    lazy_static! {
        static ref FIRST: Regex = Regex::new(r#"^([+-]?\d+(?:[.]\d*)?)[*]X\^(\d+)"#).unwrap();
    }
    lazy_static! {
        static ref AFTER: Regex = Regex::new(r#"^([+-]\d+(?:[.]\d*)?)[*]X\^(\d+)"#).unwrap();
    }

    let mut data: Data = BTreeMap::new();
    let first_cap = FIRST.captures(expression);
    let stop_point = match first_cap {
        None => return Err("Wrong format in the first element"),
        Some(cap) => {
            let mat1 = cap.get(1).unwrap();
            let mat2 = cap.get(2).unwrap();
            data.insert(
                mat2.as_str().parse::<u64>().unwrap(),
                mat1.as_str().parse::<f64>().unwrap(),
            );
            mat2.end()
        }
    };
    let mut new_s = &expression[stop_point..];
    while !new_s.is_empty() {
        let caps = AFTER.captures(new_s);
        let stop_point = match caps {
            None => {return Err("Wrong format in after element")},
            Some(cap) => {
                let mat1 = cap.get(1).unwrap();
                let mat2 = cap.get(2).unwrap();
                data.insert(
                    mat2.as_str().parse::<u64>().unwrap(),
                    mat1.as_str().parse::<f64>().unwrap(),
                );
                mat2.end()
            }
        };
        new_s = &new_s[stop_point..];
    }
    Ok(data)
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace())
}

fn solve(data: Data) {
    let &c = data.get(&0).unwrap_or(&0.0);
    let &b = data.get(&1).unwrap_or(&0.0);
    let &a = data.get(&2).unwrap_or(&0.0);
    if a == 0.0 {
        println!("The solution is:\n{:1.8}", PrettyPrintFloat(-c / b));
        return;
    }

    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        println!("Discriminant is negative, there is no solution.");
    } else if delta == 0.0 {
        let solution = -b / (2.0 * a);
        println!(
            "Discriminant is zero, the one solution is:\n{:1.8}",
            PrettyPrintFloat(solution)
        );
    } else {
        let solution1 = (-b + delta.sqrt()) / (2.0 * a);
        let solution2 = (-b - delta.sqrt()) / (2.0 * a);
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

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    match args.len() {
        2 => {
            let mut input: String = args[1].to_string();
            remove_whitespace(&mut input);
            let data: Data = reduce(input)?;
            solve(data);
            Ok(())
        }
        _ => {
            usage(&program);
            Err("program need ONE argument!")
        }
    }
}
