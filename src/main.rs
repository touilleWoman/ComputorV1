// use std::fmt;
use lazy_static::lazy_static;
use regex::{Regex, Captures};

// Ex: "5 * X^0 + 4 * X^1 - 9.3 * X^2"  => "a * X^0 + b * X^1 + c * X^2
struct Unit {
    a: f64,
    b: f64,
    c: f64,
}

// impl fmt::Display for Unit {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}{} * X^0 {} {} * X^1 {} {} * X^2",
//             self.prefix_a, self.a, self.prefix_b, self.b, self.prefix_c, self.c
//         )
//     }
// }



///c + b*x^1 + a*x^2 = 0
/// 5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0
fn solve(expression: String) {
    let (left, right) = match expression.split_once("=") {
        None => {
            panic!("Wrong input: No '=' in expression")
        }
        Some((x, y)) => (x.trim(), y.trim()),
    };
    println!("left:{}", left);
    println!("right:{}", right);
    let left_unit = extract_coefficients(left);
    let right_unit = extract_coefficients(right);
    println!("left a:{} b:{} c:{}", left_unit.a, left_unit.b, left_unit.c);
    println!("right a:{} b:{} c:{}", right_unit.a, right_unit.b, right_unit.c);
    let reduced_unit = Unit {
        a: left_unit.a - right_unit.a,
        b: left_unit.b - right_unit.b,
        c: left_unit.c - right_unit.c,
    };
    println!("reduced a:{} b:{} c:{}", reduced_unit.a, reduced_unit.b, reduced_unit.c);

}

fn get_float(caps: &Captures, name: &str) -> f64{
    let coefficient = caps.name(name).map_or("0", |m| m.as_str());
    let num = coefficient.parse::<f64>().unwrap();
    println!("coefff:{}, nb:{}", coefficient, num);
    num
}

fn extract_coefficients(expression: &str) -> Unit {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
            ^
            ((?P<a>[-]?\d+\.?\d*)[*]X\^0)?
            ((?P<b>[+-]\d+\.?\d*)[*]X\^1)?
            ((?P<c>[+-]\d+\.?\d*)[*]X\^2)?
            $
            "#
        )
        .unwrap();
    }
    let caps = RE.captures(expression).unwrap();
    let unit = Unit {
        a: get_float(&caps, "a"),
        b: get_float(&caps, "b"),
        c: get_float(&caps, "c"),
    };
    unit
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace())
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let mut input : String = args[1].to_string();
    let mut input = "5 * X^0 + 4 * X^1 - 9.3 * X^2= 1 * X^0".to_string();
    remove_whitespace(&mut input);
    solve(input);
}
