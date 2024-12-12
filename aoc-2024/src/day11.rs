use std::{collections::HashMap, fs::read_to_string, u64};

type Cache = HashMap<(u64, u64), u64>;

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let digits = stone.to_string();
        let digit_count = digits.len();
        if digit_count % 2 == 0 {
            let mid = digit_count / 2;
            let left = digits[..mid].parse().unwrap();
            let right = digits[mid..].parse().unwrap();
            vec![left, right]
        } else {
            vec![stone * 2024]
        }
    }
}

fn blinks(stone: u64, number_of_blink: u64, cache: &mut Cache) -> u64 {
    if number_of_blink == 0 {
        return 1;
    }

    if let Some(total) = cache.get(&(stone, number_of_blink)) {
        return *total;
    }

    let stones = blink(stone);
    let mut sum = 0;
    for stone in stones {
        let s = blinks(stone, number_of_blink - 1, cache);
        cache.insert((stone, number_of_blink - 1), s);
        sum += s;
    }

    sum
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    let stones: Vec<u64> = input
        .replace("\n", "")
        .split(" ")
        .filter(|nb| !nb.trim().is_empty())
        .map(|e| e.parse().expect("number string"))
        .collect();

    // Part 1
    let mut cache: Cache = HashMap::new();
    let part_1: u64 = stones
        .iter()
        .map(|stone| blinks(*stone, 25, &mut cache))
        .sum();
    println!("\nPart 1: {}", part_1);

    // Part 2
    let mut cache: Cache = HashMap::new();
    let part_2: u64 = stones
        .iter()
        .map(|stone| blinks(*stone, 75, &mut cache))
        .sum();
    println!("\nPart 2: {}", part_2);
}
