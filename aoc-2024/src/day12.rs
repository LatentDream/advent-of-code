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

// Direction needed for:
// LLL
// LFL
// LLX  <- This won't be count otherwise
type SideInfo = (Coord, Direction);

fn is_new_side(
    curr: &Coord,
    next: &Coord,
    side_horizontal: &mut HashSet<SideInfo>,
    side_vertical: &mut HashSet<SideInfo>,
) -> bool {
    // Something wrong here :(
    let direction = Direction::from_coords(curr, next).unwrap();
    match direction {
        Direction::LEFT | Direction::RIGHT => {
            let new_side = match direction {
                Direction::LEFT => next.clone(),
                Direction::RIGHT => curr.clone(),
                _ => unreachable!(),
            };
            let up = &new_side + &Direction::UP.delta();
            let down = &new_side + &Direction::DOWN.delta();
            side_vertical.insert((new_side.clone(), direction.clone()));
            !(side_vertical.contains(&(up, direction.clone()))
                || side_vertical.contains(&(down, direction)))
        }
        Direction::DOWN | Direction::UP => {
            let new_side = match direction {
                Direction::DOWN => curr.clone(),
                Direction::UP => next.clone(),
                _ => unreachable!(),
            };
            let left = &new_side + &Direction::LEFT.delta();
            let right = &new_side + &Direction::RIGHT.delta();
            side_horizontal.insert((new_side.clone(), direction.clone()));
            !(side_horizontal.contains(&(left, direction.clone()))
                || side_horizontal.contains(&(right, direction)))
        }
    }
}

fn quote_with_discount(
    coord: Coord,
    garden: &Grid<char>,
    already_process: &mut HashSet<Coord>,
    debug: &mut HashMap<char, i32>
) -> i32 {
    if already_process.contains(&coord) {
        return 0;
    }
    let plant = garden.get_at(&coord).expect("a plant");
    let mut queue = VecDeque::from(vec![coord]);
    let (mut area, mut nb_side) = (0, 0);
    // Update HashSet types to store SideInfo
    let mut side_hzt: HashSet<SideInfo> = HashSet::new();
    let mut side_vtl: HashSet<SideInfo> = HashSet::new();

    while let Some(tile) = queue.pop_front() {
        if already_process.contains(&tile) {
            continue;
        }
        already_process.insert(tile.clone());
        area += 1;
        for tile_adj in tile.adjacents() {
            match garden.get_at(&tile_adj) {
                Some(plant_adj) if plant_adj == plant => {
                    queue.push_back(tile_adj.clone());
                }
                _ => {
                    nb_side += is_new_side(&tile, &tile_adj, &mut side_hzt, &mut side_vtl) as i32;
                }
            }
        }
    }
    debug.entry(*plant)
        .and_modify(|e| *e += area * nb_side)
        .or_insert(area * nb_side);
    area * nb_side
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    // Part 1
    let mut garden = build_garden(&input);
    let mut price = 0;
    let mut already_process: HashSet<Coord> = HashSet::new();

    // Go over each path of the garden
    for coord in garden.iter_coords() {
        price += quote(coord, &mut garden, &mut already_process);
    }
    println!(" Part 1: {}$\n", price);

    // Part 2
    let mut debug: HashMap<char, i32> = HashMap::new();
    let mut garden = build_garden(&input);
    let mut price = 0;
    let mut already_process: HashSet<Coord> = HashSet::new();

    // Go over each path of the garden
    for coord in garden.iter_coords() {
        price += quote_with_discount(coord, &mut garden, &mut already_process, &mut debug);
    }
    let mut sorted_debug: Vec<_> = debug.into_iter().collect();
    sorted_debug.sort_by_key(|&(k, _)| k);
    for (plant, value) in sorted_debug {
        println!("{}: {}", plant, value);
    }
    print_garden(&garden, 'B');
    println!(" Part 2: {}$", price);
    println!(" Part 2: {}$", price);
}
