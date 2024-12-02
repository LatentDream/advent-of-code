use std::{collections::HashMap, fs};

fn main() {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (left_list, right_list): (Vec<i32>, Vec<i32>) = content
        .split("\n")
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut splited_line = line.split("   ");
            (
                splited_line.next().unwrap().parse::<i32>().unwrap(),
                splited_line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let mut nb_occur: HashMap<i32, i32> = HashMap::new();
    for number in &right_list {
        *nb_occur.entry(*number).or_insert(0) += 1;
    }

    let total: i32 = left_list
        .iter()
        .map(|low| low * nb_occur.get(&low).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", total);
}
