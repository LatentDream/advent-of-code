use std::{fs, i32};

fn get_content() -> Vec<char> {
    let content = fs::read_to_string("input.txt").expect("The file to open");
    content.chars().collect()
}

enum FindOp {
    BeginOp,
    FirstNumber,
    Separator(i32),
    SecondNumber(i32),
    CloseOp(i32, i32),
}

struct MulCompute {
    input: Vec<char>,
    state: FindOp,
    sum: i32,
    conditional: bool,
}

impl MulCompute {
    fn new(input: Vec<char>) -> Self {
        MulCompute {
            input,
            state: FindOp::BeginOp,
            sum: 0,
            conditional: true,
        }
    }

    fn compute(mut self, apply_conditionnal: bool) -> i32 {
        let mut input = self.input.iter();
        loop {
            if input.len() == 0 {
                break;
            }
            match self.state {
                FindOp::BeginOp => {
                    if input
                        .as_slice()
                        .starts_with(&['d', 'o', 'n', '\'', 't', '(', ')'])
                    {
                        self.conditional = false;
                        input.nth(6);
                    } else if input.as_slice().starts_with(&['d', 'o', '(', ')']) {
                        self.conditional = true;
                        input.nth(3);
                    } else if input.as_slice().starts_with(&['m', 'u', 'l', '(']) {
                        input.nth(3);
                        self.state = FindOp::FirstNumber
                    } else {
                        input.next();
                    }
                }
                FindOp::FirstNumber => {
                    // Build the number until `,` is found. Or error
                    let mut number_str = String::new();
                    while let Some(&c) = input.clone().peekable().peek() {
                        if c.is_ascii_digit() {
                            number_str.push(*c);
                            input.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(first_number) = number_str.parse::<i32>() {
                        self.state = FindOp::Separator(first_number);
                    } else {
                        self.state = FindOp::BeginOp;
                    }
                }
                FindOp::Separator(first_number) => {
                    match input.clone().peekable().peek() {
                        Some(&',') => {
                            // Consume the comma
                            input.next();
                            self.state = FindOp::SecondNumber(first_number);
                        }
                        _ => {
                            self.state = FindOp::BeginOp;
                        }
                    }
                }
                FindOp::SecondNumber(first_number) => {
                    let mut number_str = String::new();
                    while let Some(&c) = input.clone().peekable().peek() {
                        if c.is_ascii_digit() {
                            number_str.push(*c);
                            input.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(second_number) = number_str.parse::<i32>() {
                        self.state = FindOp::CloseOp(first_number, second_number);
                    } else {
                        self.state = FindOp::BeginOp;
                    }
                }
                FindOp::CloseOp(first_number, second_number) => {
                    if let Some(&')') = input.clone().peekable().peek() {
                        // Consume the closing parenthesis
                        input.next();
                        if !apply_conditionnal {
                            self.sum += first_number * second_number;
                        } else if apply_conditionnal && self.conditional {
                            self.sum += first_number * second_number;
                        }
                    }
                    self.state = FindOp::BeginOp;
                }
            }
        }
        self.sum
    }
}

pub fn solve() {
    let content = get_content();

    let sum = MulCompute::new(content.clone()).compute(false);
    println!("Response part 1: {}", sum);
    let sum = MulCompute::new(content).compute(true);
    println!("Response part 2: {}", sum);
}
