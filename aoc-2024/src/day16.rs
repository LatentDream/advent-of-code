use std::collections::HashSet;
use std::fs::read_to_string;
use std::{cmp::Ordering, collections::BinaryHeap};

use aoc_2024::utils::{Coord, Direction, Grid, GridOps};

struct Problem {
    grid: Grid<char>,
    end: Coord,
    start: (Coord, Direction),
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    let problem = parse_input(&input);
    println!("Start: {:?}", problem.start);
    println!("End: {:?}", problem.end);

    problem.grid.print();

    let score = traverse(&problem);
    println!("Part 1: {}", score)
}

fn traverse(problem: &Problem) -> i32 {
    // Explore the graph by putting weight on the step w/ a priority queue

    #[derive(Eq, PartialEq)]
    struct State {
        coord: Coord,
        direction: Direction,
        priority: i32,
        backtrack: Vec<Coord>,
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.priority.cmp(&other.priority)
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::with_capacity(problem.grid.len() * problem.grid[0].len());
    queue.push(State {
        coord: problem.start.0,
        direction: problem.start.1.clone(),
        priority: 0,
        backtrack: vec![], // On a turn, wait to go forward before considering the move
    });

    while let Some(val) = queue.pop() {
        let (coord, direction, priority, mut backtrack) =
            (val.coord, val.direction, val.priority, val.backtrack);

        if problem.end == coord {
            return -priority;
        }

        for coord_adj in coord.adjacents() {
            if !visited.contains(&coord_adj) {
                if problem.grid.get_at(&coord_adj) == Some(&'.') {
                    let next_direction = direction_to(&coord, &coord_adj);
                    let score = priority - turns_needed(&direction, &next_direction) * 1000 - 1;
                    if next_direction == direction {
                        visited.insert(coord);
                        visited.extend(backtrack.clone());
                        backtrack = vec![]
                    } else {
                        backtrack.push(coord_adj);
                    }
                    queue.push(State {
                        coord: coord_adj,
                        direction: next_direction,
                        priority: score,
                        backtrack: backtrack.clone(),
                    });
                }
            }
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> Problem {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());

    let mut grid = Grid::create_empty(height, width, '.');
    let mut end = None;
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coord = Coord::new(x, y);
            *grid.get_at_mut(&coord).unwrap() = match ch {
                'S' => {
                    start = Some((coord, Direction::RIGHT));
                    '.'
                }
                'E' => {
                    end = Some(coord);
                    '.'
                }
                _ => ch,
            };
        }
    }

    Problem {
        grid,
        start: start.expect("to have found the start"),
        end: end.expect("to have found the end"),
    }
}

fn turns_needed(current_dir: &Direction, next_dir: &Direction) -> i32 {
    match (current_dir, next_dir) {
        (Direction::UP, Direction::UP)
        | (Direction::DOWN, Direction::DOWN)
        | (Direction::LEFT, Direction::LEFT)
        | (Direction::RIGHT, Direction::RIGHT) => 0,
        (Direction::UP, Direction::DOWN)
        | (Direction::DOWN, Direction::UP)
        | (Direction::LEFT, Direction::RIGHT)
        | (Direction::RIGHT, Direction::LEFT) => 2,
        _ => 1,
    }
}

fn direction_to(from: &Coord, to: &Coord) -> Direction {
    if to.x > from.x {
        Direction::RIGHT
    } else if to.x < from.x {
        Direction::LEFT
    } else if to.y > from.y {
        Direction::DOWN
    } else {
        Direction::UP
    }
}
