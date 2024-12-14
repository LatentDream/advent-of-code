use aoc_2024::utils::{Coord, Direction, Grid, GridOps};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

fn build_garden(input: &str) -> Grid<char> {
    input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn print_garden(garden: &Grid<char>, target: char) {
    for row in garden {
        for cell in row {
            if *cell == target {
                print!("{}", target);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn quote(coord: Coord, garden: &Grid<char>, already_process: &mut HashSet<Coord>) -> i32 {
    if already_process.contains(&coord) {
        return 0;
    }

    let plant = garden.get_at(&coord).expect("a plant");
    let mut queue = VecDeque::from(vec![coord]);
    let (mut area, mut perimeter) = (0, 0);

    while let Some(tile) = queue.pop_front() {
        if already_process.contains(&tile) {
            continue;
        }
        already_process.insert(tile.clone());
        area += 1;

        for tile_adj in tile.adjacents() {
            match garden.get_at(&tile_adj) {
                Some(plant_adj) => {
                    if plant_adj == plant {
                        queue.push_back(tile_adj);
                    } else {
                        perimeter += 1
                    }
                }
                _ => perimeter += 1,
            }
        }
    }

    area * perimeter
}

fn count_sides(coord: &Coord, garden: &Grid<char>, plant: char) -> i32 {
    // Cound the number of sides, based on the number of corner
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut surrounding = Vec::new();

    for (dx, dy) in moves.iter() {
        let next = Coord::new(coord.x + dx, coord.y + dy);
        match garden.get_at(&next) {
            Some(p) if *p == plant => surrounding.push(1),
            _ => surrounding.push(0),
        }
    }

    let num_outside = surrounding.iter().sum::<i32>();
    let mut sides = 0;

    if num_outside == 1 {
        sides += 2; // Single connection point
    } else if num_outside == 0 {
        sides += 4; // Isolated cell
    } else if num_outside == 2 && surrounding[0] != surrounding[2] {
        sides += 1; // L-shaped corner
    }

    // Inner corners
    for i in 0..4 {
        let next_i = (i + 1) % 4;
        if surrounding[i] == 1 && surrounding[next_i] == 1 {
            let diagonal_x = coord.x + moves[i].0 + moves[next_i].0;
            let diagonal_y = coord.y + moves[i].1 + moves[next_i].1;
            let diagonal = Coord::new(diagonal_x, diagonal_y);

            if let Some(p) = garden.get_at(&diagonal) {
                if *p != plant {
                    sides += 1;
                }
            }
        }
    }

    sides
}

fn quote_with_discount(
    coord: Coord,
    garden: &Grid<char>,
    already_process: &mut HashSet<Coord>,
) -> i32 {
    if already_process.contains(&coord) {
        return 0;
    }

    let plant = garden.get_at(&coord).expect("a plant");
    let mut queue = VecDeque::from(vec![coord]);
    let (mut area, mut total_sides) = (0, 0);

    while let Some(tile) = queue.pop_front() {
        if already_process.contains(&tile) {
            continue;
        }
        already_process.insert(tile.clone());
        area += 1;
        total_sides += count_sides(&tile, garden, *plant);

        for tile_adj in tile.adjacents() {
            if let Some(plant_adj) = garden.get_at(&tile_adj) {
                if plant_adj == plant {
                    queue.push_back(tile_adj);
                }
            }
        }
    }

    area * total_sides
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");
    let garden = build_garden(&input);

    // Part 1
    let mut price = 0;
    let mut already_process: HashSet<Coord> = HashSet::new();

    // Go over each patch of the garden
    for coord in garden.iter_coords() {
        price += quote(coord, &garden, &mut already_process);
    }
    println!(" Part 1: {}$\n", price);

    // Part 2
    let mut price = 0;
    let mut already_process: HashSet<Coord> = HashSet::new();

    // Go over each patch of the garden
    for coord in garden.iter_coords() {
        price += quote_with_discount(coord, &garden, &mut already_process);
    }
    println!(" Part 2: {}$", price);
}
