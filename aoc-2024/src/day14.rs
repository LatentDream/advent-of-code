use aoc_2024::utils::Coord;
use std::fs::read_to_string;
use std::io::{self, Write};

const MAP_WIDTH: i32 = 101;
const MAP_HEIGHT: i32 = 103;

#[derive(Debug)]
pub struct Robot {
    location: Coord,
    velocity: Coord,
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    // Part 1
    let mut robots = parse_input(&input);
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.r#move();
        }
    }
    println!(
        "Product of robots in quadrants: {}",
        cound_robot_by_cadran(&robots)
    );

    // # For part 2
    // cargo run > output.txt
    // rg -A 100 -B 100 \"11111111111111111\"
    for i in 0..10000 {
        println!("Iteration number: {}", i + 100);
        debug_robots(&robots);

        for robot in robots.iter_mut() {
            robot.r#move();
        }
    }
}

impl Robot {
    fn r#move(&mut self) {
        self.location += self.velocity;
        self.location.x = (self.location.x % MAP_WIDTH + MAP_WIDTH) % MAP_WIDTH;
        self.location.y = (self.location.y % MAP_HEIGHT + MAP_HEIGHT) % MAP_HEIGHT;
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    // p=0,4 v=3,-3
    let robots_str = input.lines().filter(|l| !l.trim().is_empty());
    robots_str
        .map(|robot_str| {
            let mut parts = robot_str.split(" ");
            let loc_str = parts.next().unwrap().trim_start_matches("p=");
            let vel_str = parts.next().unwrap().trim_start_matches("v=");

            let loc: Vec<i64> = loc_str
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let vel: Vec<i64> = vel_str
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Robot {
                location: Coord::new(loc[0], loc[1]),
                velocity: Coord::new(vel[0], vel[1]),
            }
        })
        .collect()
}

fn cound_robot_by_cadran(robots: &[Robot]) -> i32 {
    let mut cadran = [0, 0, 0, 0];
    let middle_x = MAP_WIDTH / 2;
    let middle_y = MAP_HEIGHT / 2;

    for robot in robots {
        let x = robot.location.x;
        let y = robot.location.y;

        match (x.cmp(&middle_x), y.cmp(&middle_y)) {
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => cadran[0] += 1, // Top-right
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => cadran[1] += 1,    // Top-left
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => cadran[2] += 1, // Bottom-left
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => cadran[3] += 1, // Bottom-right
            _ => {} // On the dividing lines, not counted in any quadrant
        }
    }
    cadran[0] * cadran[1] * cadran[2] * cadran[3]
}

pub fn debug_robots(robots: &[Robot]) {
    let mut map = vec![vec![0; MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    // Count robots on each tile
    for robot in robots {
        let x = robot.location.x as usize;
        let y = robot.location.y as usize;
        map[y][x] += 1;
    }

    // Print the map
    for row in map {
        for count in row {
            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
    println!();
}
