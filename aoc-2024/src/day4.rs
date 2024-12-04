use std::fs;

fn get_input() -> Vec<Vec<char>> {
    let content = fs::read_to_string("input.txt").expect("the f*cking file to open");
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn eq_xmax(input: &String) -> bool {
    input.eq("XMAS")
}

fn get_string_in_direction(
    puzzle: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    direction_step: (i32, i32),
) -> Option<String> {
    let (di, dj) = direction_step;
    let mut accumulator = Vec::with_capacity(4);
    for k in 0..4 {
        let i_loc = i as i32 + (k * di);
        let j_loc = j as i32 + (k * dj);
        if i_loc < 0 || j_loc < 0 {
            return None;
        }
        let (i_loc, j_loc) = (i_loc as usize, j_loc as usize);
        if i_loc >= puzzle.len() || j_loc >= puzzle[0].len() {
            return None;
        }
        accumulator.push(puzzle[i_loc][j_loc]);
    }
    Some(String::from_iter(accumulator))
}

pub fn solve() {
    let puzzle = get_input();

    let directions = [
        ((0, -1), "Left"),  // Left
        ((0, 1), "Right"),  // Right
        ((-1, 0), "Down"),  // Down
        ((1, 0), "Up"),     // Up
        ((-1, -1), "Left"), // Left-Down
        ((1, -1), "Up"),    // Left-Up
        ((1, 1), "Right"),  // Right-Up
        ((-1, 1), "Down"),  // Right-Down
    ];

    let mut total_found = 0;

    for (i, row) in puzzle.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'X' {
                for ((di, dj), name) in directions.iter() {
                    if let Some(string) = get_string_in_direction(&puzzle, i, j, (*di, *dj)) {
                        // println!("{}: {}", name, string);
                        if eq_xmax(&string) {
                            total_found += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", total_found);
}
