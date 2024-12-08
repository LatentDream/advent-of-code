use std::fs;

fn read_input() -> Vec<(u64, Vec<u64>)> {
    let content = fs::read_to_string("input.txt").expect("The file to open");

    let problem_inputs: Vec<(u64, Vec<u64>)> = content
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut splited_line = line.split(":");
            (
                splited_line.next().unwrap().parse::<u64>().unwrap(),
                splited_line
                    .next()
                    .unwrap()
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|e| e.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect();

    problem_inputs
}

type Operator = fn(u64, u64) -> u64;

fn recursive_compute(target: u64, sum: u64, inputs: &[u64], operators: &[Operator]) -> bool {
    if inputs.is_empty() {
        return sum == target;
    }
    operators.iter().any(
        |op| recursive_compute(target, op(sum, inputs[0]), &inputs[1..], operators)
    )
}

fn evaluate(inputs: &Vec<(u64, Vec<u64>)>, operators: &[Operator]) -> u64 {
    let mut total = 0;
    for (target, inputs) in inputs {
        if recursive_compute(*target, inputs[0], &inputs[1..], &operators) {
            total += target;
        }
    }
    total
}

pub fn solve() {
    let inputs = read_input();
    let operators: Vec<Operator> = vec![
        |a, b| a + b,
        |a, b| a * b,
    ];
    println!("Part 1: {}", evaluate(&inputs, &operators));


    let operators: Vec<Operator> = vec![
        |a, b| a + b,
        |a, b| a * b,
        |a, b| format!("{}{}",a, b).parse::<u64>().unwrap()
    ];
    println!("Part 2: {}", evaluate(&inputs, &operators));
}
