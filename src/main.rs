use std::env;
use regex::Regex;

/// Ex: 4 * X^2 => nb = 4, variable = X, power = 2
struct Unit <'a>{
    nb : 'a i32,
    variable : 'a char,
    power: 'a i32
}

impl<'a> fmt::Display for Unit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * {}^{}", self.nb, self.variable, self.power)
    }
}

fn extract_variable(input: &str) -> Option<&str> {
    let re_alpha= Regex::new(r"[[:alpha:]]").unwrap();
    re_alpha.captures(input).and_then(|cap|
        {cap.name("variable").map(|variable| variable.as_str())})
}


///c + b*x^1 + a*x^2 = 0
/// 5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0
fn solve(expression: &str){
    let (left, right) = match expression.split_once("=") {
        None => {panic!("Wrong input: need one '=' in expression")},
        Some((x, y)) => (x, y),
    };
    let right_pattern = Regex::new(r"^(?P<number>(\d|.)+)\*X\^(?P<power>[0-2])")?;
    let caps = right_pattern.captures(right).unwrap();
    println!("number|{:?}| power |{:?}| ", caps["number"], caps["power"]);

}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace())
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let mut input : String = args[1].to_string();
    let input = "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0";
    remove_whitespace(&mut input);
    let re = Regex::new(
        r"(-{0,1}(\d+.))"
    )?;


}