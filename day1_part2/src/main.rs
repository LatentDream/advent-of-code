use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;  


fn main() {
    let number_str = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    // let file = File::open("../day1_part1/src/input.txt").unwrap();
    let file = File::open("src/input.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut total_value: u128 = 0;

    for line in buf_reader.lines() {
        
        let content = line.unwrap();
        let mut idx_first = content.len();
        let mut value_first = 0;
        let mut idx_last: i32 = -1;
        let mut value_last = 0;

        // Check for number
        for (i, char) in content.chars().enumerate() {
            if char.is_digit(10) {
                let number = char.to_digit(10).unwrap();
                if i < idx_first {
                    idx_first = i;
                    value_first = number * 10;
                }
                if i as i32 > idx_last {
                    idx_last = i as i32;
                    value_last = number;
                }
            }
        }
        // Check for string number
        for (i, str_number) in number_str.iter().enumerate() {
            let idx = content.find(str_number);
            if idx.is_some() {
                let idx = idx.unwrap();
                if idx < idx_first {
                    idx_first = idx;
                    value_first = i as u32 * 10;
                }
                // Check if the is another occurrence of the same number
                fn recursive_search(content: &str, str_number: &str) -> usize {
                    let idx = content.find(str_number);
                    if idx.is_some() {
                        let idx = idx.unwrap();
                        return idx + 1 + recursive_search(&content[idx+1..], str_number);
                    } else {
                        return 0;
                    }
                }
                let idx = idx + recursive_search(&content[(idx+1) as usize..], str_number);
                if idx as i32 > idx_last {
                    idx_last = idx as i32;
                    value_last = i as u32;
                }
            }
        }

        total_value += value_first as u128 + value_last as u128;

    }

    println!("Total value: {}", total_value);   

}

