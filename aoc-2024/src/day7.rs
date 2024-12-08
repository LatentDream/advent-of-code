use std::fs;

fn read_input() -> Vec<(i64, Vec<i64>)> {
    let content = fs::read_to_string("input.txt").expect("The file to open");

    let problem_inputs: Vec<(i64, Vec<i64>)> = content
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut splited_line = line.split(":");
            (
                splited_line.next().unwrap().parse::<i64>().unwrap(),
                splited_line
                    .next()
                    .unwrap()
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|e| e.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect();

    problem_inputs
}

fn could_be_true(target: i64, inputs: Vec<i64>) -> bool {
    fn compute(target: i64, sum: i64, inputs: &[i64]) -> bool {
        if sum == target {
            return true;
        }
        if inputs.len() == 0 || sum > target {
            return false;
        }

        compute(target, sum + inputs[0], &inputs[1..])
            || compute(target, sum * inputs[0], &inputs[1..])
    }

    return compute(target, inputs[0], &inputs[1..]);
}

pub fn solve() {
    let inputs = read_input();

    let mut total = 0;
    for (target, inputs) in inputs {
        if could_be_true(target, inputs) {
            total += target;
        }
    }
}
