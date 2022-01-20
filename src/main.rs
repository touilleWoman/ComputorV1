use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fmt;

// Ex: "5 * X^0 + 4 * X^1 - 9.3 * X^2"  => "c * X^0 + b * X^1 + a * X^2
struct Unit {
    a: f64,
    b: f64,
    c: f64,
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
        write!(
            f,
            "{} = 0",
            expression.trim_start_matches("+ "),
        )
    }
}

fn reduce(expression: String) -> Unit {
    let (left, right) = match expression.split_once("=") {
        None => {
            panic!("Wrong input: No '=' in expression")
        }
        Some((x, y)) => (x.trim(), y.trim()),
    };
    let left_unit = extract_coefficients(left);
    let right_unit = extract_coefficients(right);
    let reduced_unit = Unit {
        a: left_unit.a - right_unit.a,
        b: left_unit.b - right_unit.b,
        c: left_unit.c - right_unit.c,
    };
    let degree = {
        if reduced_unit.a != 0.0 {
            "2"
        } else if reduced_unit.b != 0.0 {
            "1"
        } else {
            "0"
        }
    };
    println!(
        "Reduced form: {}\nPolynomial degree: {}",
        reduced_unit, degree
    );
    reduced_unit
}

fn get_float(caps: &Captures, name: &str) -> f64 {
    let coefficient = caps.name(name).map_or("0", |m| m.as_str());
    let num = coefficient.parse::<f64>().unwrap();
    // println!("coefff:{}, nb:{}", coefficient, num);
    num
}

fn extract_coefficients(expression: &str) -> Unit {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
            ^
            ((?P<c>[-]?\d+\.?\d*)[*]X\^0)?
            ((?P<b>[+-]?\d+\.?\d*)[*]X\^1)?
            ((?P<a>[+-]?\d+\.?\d*)[*]X\^2)?
            $
            "#
        )
        .unwrap();
    }
    let caps = RE.captures(expression).unwrap();
    let unit = Unit {
        c: get_float(&caps, "c"),
        b: get_float(&caps, "b"),
        a: get_float(&caps, "a"),
    };
    unit
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace())
}

fn solve(data: Unit){
    let delta = data.b * data.b - 4.0 * data.a * data.c;
    if delta < 0.0 {
        println!("Discriminant is negative, there is no solution.");
    } else if delta == 0.0 {
        println!("Discriminant is zero, the one solution is:\n{}", -data.b / (2.0 * data.a));
    } else {
        let solution1 = (-data.b + delta.sqrt())/ (2.0 * data.a);
        let solution2 = (-data.b - delta.sqrt())/ (2.0 * data.a);
        println!("Discriminant is strictly positive, the two solutions are:\n{}\n{}", solution1, solution2)
    }
}

fn main() {
    assert!(0.00 ==  -0.0);
    // let args: Vec<String> = env::args().collect();
    // let mut input : String = args[1].to_string();
    let mut input = "4 * X^1 - 9.3 * X^2= 1 * X^1".to_string();
    // let mut input = "5 * X^0 + 4 * X^1 - 9.3 * X^2= 1 * X^0".to_string();
    remove_whitespace(&mut input);
    let data = reduce(input);
    solve(data);
}
