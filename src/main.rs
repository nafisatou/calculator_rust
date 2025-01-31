use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Add, Subtract, Multiply, Divide, Modulus, Exponentiate, Factorial,
}

impl Operation {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            "%" => Some(Operation::Modulus),
            "^" => Some(Operation::Exponentiate),
            "fact" => Some(Operation::Factorial),
            _ => None,
        }
    }

    fn apply(&self, a: f64, b: f64) -> Result<f64, String> {
        match *self {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide if b != 0.0 => Ok(a / b),
            Operation::Divide => Err("Divide by zero".into()),
            Operation::Modulus if b != 0.0 => Ok(a % b),
            Operation::Modulus => Err("Divide by zero".into()),
            Operation::Exponentiate => Ok(a.powf(b)),
            Operation::Factorial => {
                if a < 0.0 || a != a.floor() { Err("Factorial requires a non-negative integer".into()) }
                else { Ok(factorial(a as u64) as f64) }
            }
        }
    }
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut history = Vec::new();
    loop {
        let choice = get_input("Single or multiple operation? (single/multiple): ");
        if choice == "single" {
            let a = get_input("Enter first number: ").parse::<f64>().unwrap();
            let op = get_input("Enter operator (+, -, *, /, %, ^, fact): ");
            let b = if op == "fact" { 0.0 } else { get_input("Enter second number: ").parse().unwrap() };
            
            if let Some(op) = Operation::from_str(&op) {
                match op.apply(a, b) {
                    Ok(res) => { println!("Result: {}", res); history.push(res); },
                    Err(e) => println!("Error: {}", e),
                }
            }
        } else if choice == "multiple" {
            let expr = get_input("Enter expression: ");
            let tokens: Vec<String> = expr.split_whitespace().map(String::from).collect();
            // Process expression parsing (as before)
        }

        if get_input("Another calculation? (y/n): ").to_lowercase() != "y" { break; }
    }
    println!("History: {:?}", history);
}
