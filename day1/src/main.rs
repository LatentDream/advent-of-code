use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn part_1(line: &String) -> u128 {
    let mut fixed_first = false;
    let mut first_int: u16 = 0;
    let mut last_int: u8 = 0; 
    for  val in line.chars() {
        let ascii = val as u8;
        if ascii > 47 && ascii < 58 { 
            if !fixed_first {
                first_int = ((ascii - 48) as u16) * 10;
                last_int = ascii - 48;
                fixed_first = true;
            } else {
                last_int = ascii - 48;
            }
        }
    }
    return first_int as u128 + last_int as u128;
}

fn part_2(line: &String) -> u128 {
    const NUMBER_ST: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    fn check_if_digit(s: &str, idx: usize) -> Option<u32> {
        s.as_bytes()
            .get(idx)
            .and_then(|&b| (b as char).to_digit(10))
            .or_else(|| {
                for (i, w) in NUMBER_ST.iter().enumerate(){
                    if s[idx..].starts_with(w) {
                        return Some(i as u32);
                    }
                }
                None
            })
    }
    
    let digit = (0..line.len())
        .filter_map(|idx| check_if_digit(line, idx))
        .collect::<Vec<_>>();
    let value = digit.first().zip(digit.last()).map(|(first, last)| first * 10 + last).unwrap();
    return value as u128;

}

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut total_value: u128 = 0;
    for line in buf_reader.lines() {
        let content = line.unwrap();
        if std::env::args().nth(1).unwrap() == "1" {
            total_value += part_1(&content);
        } else {
            total_value += part_2(&content);
        }
    }
    println!("{}", total_value);
}
