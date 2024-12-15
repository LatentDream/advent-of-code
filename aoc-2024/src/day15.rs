use core::panic;
use std::{collections::HashSet, fs::read_to_string};

use aoc_2024::utils::{Coord, Direction, Grid, GridOps};

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    let (mut map, directions) = parse_input(&input);
    for direction in directions.iter() {
        map.try_move(direction);
    }

    let total: i32 = map.boxes.iter().map(|coord| 100 * coord.y + coord.x).sum();
    println!("Part 1: {}", total);
}

#[derive(Clone)]
struct Transaction {
    robot: Coord,
    boxes: HashSet<Coord>,
}

struct Map {
    grid: Grid<char>,
    robot: Coord,
    boxes: HashSet<Coord>,
}

impl Map {
    fn try_move(&mut self, direction: &Direction) -> Option<()> {
        // Start a transaction
        let mut transaction = Transaction {
            robot: self.robot,
            boxes: self.boxes.clone(),
        };

        // Move the robot
        let mut new_position = transaction.robot + direction.delta();
        if !self.is_valid_position(&new_position) {
            return None; // There is a wall, cancel transaction
        }

        transaction.robot = new_position;
        transaction.boxes.remove(&new_position);

        // Push the boxes
        while self.boxes.contains(&new_position) {
            let next_position = new_position + direction.delta();
            if !self.is_valid_position(&next_position) {
                return None; // There is a wall, cancel transaction
            }
            transaction.boxes.insert(next_position);
            new_position = next_position;
        }

        // Commit transaction
        self.robot = transaction.robot;
        self.boxes = transaction.boxes;
        Some(())
    }

    fn is_valid_position(&self, coord: &Coord) -> bool {
        if let Some(tile) = self.grid.get_at(coord) {
            return match tile {
                '.' => true,
                _ => false,
            };
        }
        false
    }

    fn display(&self) {
        let (max_x, _) = self.grid.get_dimensions();
        for coord in self.grid.iter_coords() {
            let (x, y) = (coord.x, coord.y);
            let coord = Coord { x, y };
            let char_to_print = if coord == self.robot {
                '@'
            } else if self.boxes.contains(&coord) {
                'O'
            } else {
                // Get the underlying grid cell (floor or wall)
                *self.grid.get_at(&coord).unwrap_or(&'#')
            };
            print!("{}", char_to_print);
            if x == max_x as i32 - 1 {
                println!(); // New line at the end of each row
            }
        }
        println!(); // Extra line for better readability between moves
    }
}

fn parse_input(input: &str) -> (Map, Vec<Direction>) {
    let (raw_map, raw_input) = input
        .split_once("\n\n")
        .expect("input to contain map and instructions separated by blank line");

    let map = parse_map(&raw_map);
    let directions: Vec<Direction> = parse_directions(&raw_input);
    (map, directions)
}

fn parse_map(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());

    let mut grid = Grid::create_empty(width, height, '.');
    let mut robot = Coord { x: 0, y: 0 };
    let mut boxes = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coord = Coord::new(x, y);
            if let Some(cell) = grid.get_at_mut(&coord) {
                match ch {
                    '@' => {
                        *cell = '.'; // Floor under robot
                        robot = coord;
                    }
                    'O' => {
                        *cell = '.'; // Floor under box
                        boxes.insert(coord);
                    }
                    _ => *cell = ch,
                }
            }
        }
    }

    Map { grid, robot, boxes }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|char| match char {
            '<' => Direction::LEFT,
            '>' => Direction::RIGHT,
            '^' => Direction::UP,
            'v' => Direction::DOWN,
            _ => panic!("Unsupported direction '{}'", char),
        })
        .collect()
}
