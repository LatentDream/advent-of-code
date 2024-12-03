use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("The f***ing file to open");

    let reports: Vec<Vec<i32>> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Only number as input"))
                .collect()
        })
        .collect();

    let mut nb_safe_report = 0;

    for report in reports {
        let mut iter = report.iter();
        let mut increase: Option<bool> = None;
        let mut prev_val = iter.next().expect("None empty report :(");
        loop {
            let curr = iter.next();
            if let Some(curr_val) = curr {
                if increase.is_none() {
                    if curr_val > prev_val {
                        increase = Some(true);
                    } else {
                        increase = Some(false);
                    }
                }
                let diff = curr_val - prev_val;
                if increase.expect("Not to have remove the setter above") {
                    if 1 <= diff && diff <= 3 {
                        prev_val = curr_val;
                        continue;
                    }
                } else {
                    if 1 <= -diff && -diff <= 3 {
                        prev_val = curr_val;
                        continue;
                    }
                }
                break;
            } else {
                nb_safe_report += 1;
                break;
            }
        }
    }

    println!("Response {}", nb_safe_report);
}
