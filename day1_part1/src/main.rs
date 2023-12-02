use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Goal: Little refresh on ASCII 

fn main() {
    
    let file = File::open("src/input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    let mut total_value: u128 = 0;
    
    loop {
        let num_bytes = buf_reader.read_line(&mut content).unwrap();
        if num_bytes == 0 {
            break;
        }
        let mut fixed_first = false;
        let mut first_int: u16 = 0;
        let mut last_int: u8 = 0; 
        for  val in content.chars() {
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
        total_value += (first_int as u128) + (last_int as u128);
        content.clear();
    }

    println!("{}", total_value);
}
