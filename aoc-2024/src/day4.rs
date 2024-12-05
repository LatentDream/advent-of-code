use std::fs;

fn get_input() -> Vec<Vec<char>> {
    let content = fs::read_to_string("input.txt").expect("the f*cking file to open");
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn eq_xmax(input: String) -> bool {
    input.eq("XMAS")
}

fn eq_max(input: String) -> bool {
    input.eq("MAS") || input.eq("SAM")
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

fn get_strings_in_diago_from_center(
    puzzle: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> Option<(String, String)> {
    if i < 1 || i >= puzzle.len() - 1 || j < 1 || j >= puzzle[0].len() - 1 {
        return None;
    }
    let diag1 = format!(
        "{}{}{}",
        puzzle[i - 1][j - 1],
        puzzle[i][j],
        puzzle[i + 1][j + 1]
    );
    let diag2 = format!(
        "{}{}{}",
        puzzle[i - 1][j + 1],
        puzzle[i][j],
        puzzle[i + 1][j - 1]
    );

    Some((diag1, diag2))
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

    let mut total_found_part_1 = 0;

    for (i, row) in puzzle.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'X' {
                for ((di, dj), _name) in directions.iter() {
                    if let Some(string) = get_string_in_direction(&puzzle, i, j, (*di, *dj)) {
                        if eq_xmax(string) {
                            total_found_part_1 += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", total_found_part_1);

    let mut total_found_part_2 = 0;
    for (i, row) in puzzle.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'A' {
                if let Some((diag1, diag2)) = get_strings_in_diago_from_center(&puzzle, i, j) {
                    if eq_max(diag1) && eq_max(diag2) {
                        total_found_part_2 += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", total_found_part_2);
}
