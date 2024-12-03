use std::{fs, i32, u32};

type Report = Vec<i32>;

fn get_reports() -> Vec<Report> {
    let content = fs::read_to_string("input.txt").expect("The file to open");

    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("only number as input"))
                .collect()
        })
        .collect()
}

fn is_increasing(report: &Report) -> bool {
    let (increases, decreases) =
        report
            .windows(2)
            .fold((0, 0), |(inc, dec), window| match window {
                [prev, curr] if curr > prev => (inc + 1, dec),
                [prev, curr] if curr < prev => (inc, dec + 1),
                _ => (inc, dec),
            });
    increases > decreases
}

fn is_valid(report: &Report) -> u32 {
    let is_increasing = is_increasing(&report);
    let mut iter = report.iter();
    let mut prev_val = iter.next().expect("None empty report :(");
    'outer: loop {
        let curr = iter.next();
        if let Some(curr_val) = curr {
            let diff = curr_val - prev_val;
            if is_increasing {
                if 1 <= diff && diff <= 3 {
                    prev_val = curr_val;
                    continue 'outer;
                }
            } else {
                if 1 <= -diff && -diff <= 3 {
                    prev_val = curr_val;
                    continue 'outer;
                }
            }
            break;
        } else {
            return 1;
        }
    }
    0
}

fn is_valid_with_one_removal(report: &Report) -> u32 {
    if is_valid(report) == 1 {
        return 1;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_valid(&modified_report) == 1 {
            return 1;
        }
    }
    0
}

pub fn solve() {
    let reports = get_reports();
    let nb_valid_part1 = reports.iter().fold(0, |acc, report| acc + is_valid(report));
    let nb_valid_part2 = reports
        .iter()
        .fold(0, |acc, report| acc + is_valid_with_one_removal(report));

    println!("Response part 1: {}", nb_valid_part1);
    println!("Response part 2: {}", nb_valid_part2);
}
