use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Add, Subtract, Multiply, Divide, Modulus, Exponentiate,
}

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Subtract),
            '*' => Some(Operation::Multiply),
            '/' => Some(Operation::Divide),
            '%' => Some(Operation::Modulus),
            '^' => Some(Operation::Exponentiate),
            _ => None,
        }
    }

    fn apply(&self, a: f64, b: f64) -> Result<f64, String> {
        match *self {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide if b != 0.0 => Ok(a / b),
            Operation::Divide => Err("Cannot divide by zero".to_string()),
            Operation::Modulus if b != 0.0 => Ok(a % b),
            Operation::Modulus => Err("Cannot divide by zero".to_string()),
            Operation::Exponentiate => Ok(a.powf(b)),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_input(input: &str) -> Result<Vec<String>, String> {
    let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
    if parts.len() % 2 == 0 { Err("Invalid expression".to_string()) } else { Ok(parts) }
}

fn evaluate_expression(tokens: Vec<String>) -> Result<f64, String> {
    let mut result = tokens[0].parse::<f64>().map_err(|_| "Invalid number")?;
    for i in (1..tokens.len()).step_by(2) {
        let op = Operation::from_char(tokens[i].chars().next().unwrap()).ok_or("Invalid operator")?;
        let operand = tokens[i + 1].parse::<f64>().map_err(|_| "Invalid number")?;
        result = op.apply(result, operand)?;
    }
    Ok(result)
}

fn main() {
    let mut history = Vec::new();
    loop {
        let choice = get_input("Single or multiple operation? (single/multiple): ");
        if choice == "single" {
            let a = get_input("Enter first number: ").parse::<f64>().unwrap();
            let op = get_input("Enter operator (+, -, *, /, %, ^): ").chars().next().unwrap();
            let b = get_input("Enter second number: ").parse::<f64>().unwrap();
            if let Some(op) = Operation::from_char(op) {
                match op.apply(a, b) {
                    Ok(result) => { println!("Result: {}", result); history.push(result); },
                    Err(e) => println!("Error: {}", e),
                }
            }
        } else if choice == "multiple" {
            let expr = get_input("Enter expression (e.g., '10 + 5 * 2'): ");
            match parse_input(&expr).and_then(evaluate_expression) {
                Ok(result) => { println!("Result: {}", result); history.push(result); },
                Err(e) => println!("Error: {}", e),
            }
        }

        if get_input("Another calculation? (y/n): ").to_lowercase() != "y" { break; }
    }
    println!("History: {:?}", history);
}
