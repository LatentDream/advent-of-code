use aoc_2024::utils::{Coord, Grid, GridOps};
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn build_garden(input: &str) -> Grid<char> {
    input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn quote(coord: Coord, garden: &mut Grid<char>, already_process: &mut HashSet<Coord>) -> i32 {
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

        // Check the current tile for it's perimeter
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

    println!("Area {} = {}$", plant, area*perimeter);
    area * perimeter
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");
    let mut garden = build_garden(&input);

    let mut price = 0;

    // Go over each path of the garden
    let (x_dim, y_dim) = garden.get_dimensions();
    let mut already_process: HashSet<Coord> = HashSet::new();
    for x in 0..x_dim {
        for y in 0..y_dim {
            let coord = Coord::new(x, y);
            price += quote(coord, &mut garden, &mut already_process);
        }
    }

    println!("{:?}", price);
}
