use aoc_2024::utils::{Coord, Direction, Grid, GridOps};
use core::panic;
use std::io::{self, Write};
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    let (mut map, directions) = parse_input(&input, false);
    for direction in directions.iter() {
        map.try_move(direction);
    }
    let total: i32 = map
        .boxes
        .iter()
        .map(|r#box| 100 * r#box.left.y + r#box.left.x)
        .sum();
    println!("Part 1: {}", total);

    let (mut map, directions) = parse_input(&input, true);
    for direction in directions.iter() {
        map.try_move(direction);
    }
    let total: i32 = map
        .boxes
        .iter()
        .map(|r#box| 100 * r#box.left.y + r#box.left.x)
        .sum();
    println!("Part 2: {}", total);
}

#[derive(Clone)]
struct Transaction {
    robot: Coord,
    boxes: HashSet<BoxPosition>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct BoxPosition {
    left: Coord,
    right: Coord,
    id: u32,
}

struct Map {
    grid: Grid<char>,
    robot: Coord,
    boxes: HashSet<BoxPosition>,
    is_double_width: bool,
}

impl Map {
    fn try_move(&mut self, direction: &Direction) -> Option<()> {
        let mut transaction = Transaction {
            robot: self.robot,
            boxes: self.boxes.clone(),
        };

        // Move the robot
        let new_position = transaction.robot + direction.delta();
        if !self.is_valid_position(&new_position) {
            return None; // rollback transaction
        }
        transaction.robot = new_position;

        // Check for box collision
        let mut processed_boxes = HashSet::new();
        let mut blocking_boxes = VecDeque::new();
        if let Some(blocking_box) = self.find_box_at(&new_position) {
            blocking_boxes.push_back(blocking_box.clone());
            processed_boxes.insert(blocking_box.id);
        }

        while let Some(blocking_box) = blocking_boxes.pop_front() {
            let box_delta = direction.delta();
            let new_left = blocking_box.left + box_delta;
            let new_right = blocking_box.right + box_delta;

            // Rollback if not valid
            if !self.is_valid_position(&new_left) || !self.is_valid_position(&new_right) {
                return None;
            }

            // Check if there's box(s) in the way
            let left_box = self
                .find_box_at(&new_left)
                .filter(|box_pos| !processed_boxes.contains(&box_pos.id));
            let right_box = self
                .find_box_at(&new_right)
                .filter(|box_pos| !processed_boxes.contains(&box_pos.id));
            match (left_box, right_box) {
                (Some(left), Some(right)) if left == right => {
                    blocking_boxes.push_back(left.clone());
                    processed_boxes.insert(left.id);
                }
                (Some(left), Some(right)) => {
                    blocking_boxes.push_back(left.clone());
                    blocking_boxes.push_back(right.clone());
                    processed_boxes.insert(left.id);
                    processed_boxes.insert(right.id);
                }
                (Some(box_pos), None) | (None, Some(box_pos)) => {
                    blocking_boxes.push_back(box_pos.clone());
                    processed_boxes.insert(box_pos.id);
                }
                (None, None) => {}
            }

            transaction.boxes.remove(&blocking_box);
            transaction.boxes.insert(BoxPosition {
                left: new_left,
                right: new_right,
                id: blocking_box.id,
            });
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

    fn find_box_at(&self, coord: &Coord) -> Option<BoxPosition> {
        self.boxes
            .iter()
            .find(|box_pos| box_pos.left == *coord || box_pos.right == *coord)
            .cloned()
    }

    fn display(&self) {
        let mut current_y = 0;

        for coord in self.grid.iter_coords() {
            // Print newline when y coordinate changes
            if coord.y > current_y {
                println!();
                current_y = coord.y;
            }

            // Determine what character to print at this position
            let display_char = if coord == self.robot {
                '@'
            } else {
                let box_part = self.boxes.iter().find(|box_pos| {
                    if self.is_double_width {
                        coord == box_pos.left || coord == box_pos.right
                    } else {
                        coord == box_pos.left
                    }
                });

                match (self.is_double_width, box_part) {
                    (true, Some(box_pos)) => {
                        if coord == box_pos.left {
                            '['
                        } else {
                            ']'
                        }
                    }
                    (false, Some(_)) => 'O',
                    _ => *self.grid.get_at(&coord).unwrap_or(&'.'),
                }
            };

            print!("{}", display_char);
        }
        // Final newline after the last row
        println!();
    }
}

fn pause() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn parse_input(input: &str, double_width: bool) -> (Map, Vec<Direction>) {
    let (raw_map, raw_input) = input
        .split_once("\n\n")
        .expect("input to contain map and instructions separated by blank line");

    let map = parse_map(&raw_map, double_width);
    let directions: Vec<Direction> = parse_directions(&raw_input);
    (map, directions)
}

fn parse_map(input: &str, double_width: bool) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len()) * (double_width as usize + 1);

    let mut grid = Grid::create_empty(height, width, '.');
    let mut robot = Coord { x: 0, y: 0 };
    let mut boxes = HashSet::new();
    let mut box_id = 0;

    // First pass: create basic grid
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coord = Coord::new(x, y);
            if double_width {
                let wide_x = x * 2;
                let wide_coord = Coord::new(wide_x, y);
                let next_wide_coord = Coord::new(wide_x + 1, y);

                if let Some(cell) = grid.get_at_mut(&wide_coord) {
                    match ch {
                        '@' => {
                            *cell = '.';
                            robot = wide_coord;
                        }
                        'O' => {
                            *cell = '.';
                            boxes.insert(BoxPosition {
                                left: wide_coord,
                                right: next_wide_coord,
                                id: box_id,
                            });
                            box_id += 1;
                        }
                        _ => *cell = ch,
                    }
                }
                if let Some(cell) = grid.get_at_mut(&next_wide_coord) {
                    match ch {
                        '@' => {
                            *cell = '.';
                        }
                        'O' => {
                            *cell = '.';
                        }
                        _ => *cell = ch,
                    }
                }
            } else {
                if let Some(cell) = grid.get_at_mut(&coord) {
                    match ch {
                        '@' => {
                            *cell = '.'; // Floor under robot
                            robot = coord;
                        }
                        'O' => {
                            *cell = '.'; // Floor under box
                            boxes.insert(BoxPosition {
                                left: coord,
                                right: coord,
                                id: box_id,
                            });
                            box_id += 1;
                        }
                        _ => *cell = ch,
                    }
                }
            }
        }
    }

    Map {
        grid,
        robot,
        boxes,
        is_double_width: double_width,
    }
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
