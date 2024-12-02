use std::fs;

fn main() {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (mut collection_low, mut collection_high): (Vec<i32>, Vec<i32>) = content
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

    collection_low.sort();
    collection_high.sort();

    let total: i32 = collection_low
        .iter()
        .enumerate()
        .map(|(i, low)| (collection_high[i] - low).abs())
        .sum();

    println!("Total distance: {}", total);
}
