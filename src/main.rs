use float_pretty_print::PrettyPrintFloat;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::collections::BTreeMap;


type Data = BTreeMap<u8, f64>;


fn print_reduced(reduced: &Data){
    let mut expression : String = String::new();
    for (&key, &value) in reduced.iter(){
        if value > 0.0 {
            expression.push_str(&format!("+ {} * X^{} ", value.to_string(), key));
        } else {
            expression.push_str(&format!("- {} * X^{} ", (-value).to_string(), key));
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
    let degree = match reduced.keys().max(){
        None => return Err("No power in expression"),
        Some(x) => *x,
    };
    print_reduced(&reduced);

    if degree > 2 {
        return Err("The polynomial degree is stricly greater than 2, I can't solve.");
    } else {
        println!(
            "Polynomial degree: {}",
            degree
        );
    }
    Ok(reduced)
}

fn extract_coefficients(expression: &str) -> Result<Data, &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"([+-]?\d+\.?\d*)[*]X\^(\d{1,2})"#)
        .unwrap();
    }
    let caps = RE.captures_iter(expression);
    let mut data :Data = BTreeMap::new();
    for cap in caps{
        match data.insert(cap[2].parse::<u8>().unwrap(), cap[1].parse::<f64>().unwrap()) {
            None => continue,
            _ => return Err("Wrong format: Duplicate coefficant is not allowed"),
        }
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
        println!(
            "The solution is:\n{:1.8}",
            PrettyPrintFloat(-c / b)
        );
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
