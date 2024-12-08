use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use aoc_2024::utils::Coord;

type Frequency = char;

pub struct CityMap {
    map: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
}

pub fn build_city_map(input: String) -> CityMap {
    let map: Vec<Vec<char>> = input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let max_y = map.len() as i32;
    let max_x = map[0].len() as i32;

    CityMap { map, max_x, max_y }
}

fn find_antennas(city_map: &CityMap) -> HashMap<Frequency, HashSet<Coord>> {
    let mut antennas: HashMap<Frequency, HashSet<Coord>> = HashMap::new();
    for (y, line) in city_map.map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '.' {
                let coord = Coord::new(x, y);
                antennas
                    .entry(*c)
                    .or_insert_with(|| HashSet::new())
                    .insert(coord);
            }
        }
    }
    antennas
}

fn is_within_bounds(coord: &Coord, city_map: &CityMap) -> bool {
    coord.x >= 0 && coord.x < city_map.max_x && coord.y >= 0 && coord.y < city_map.max_y
}

type AntinodesFinder = fn(&Coord, &Coord, &CityMap) -> Vec<Coord>;

fn find_antinodes_part1(antenna_1: &Coord, antenna_2: &Coord, city_map: &CityMap) -> Vec<Coord> {
    let delta = antenna_2 - antenna_1;

    let antinodes = vec![antenna_2 + &delta, antenna_1 - &delta];
    antinodes
        .into_iter()
        .filter(|coord| is_within_bounds(coord, city_map))
        .collect()
}

fn find_antinodes_part2(antenna_1: &Coord, antenna_2: &Coord, city_map: &CityMap) -> Vec<Coord> {
    let delta = antenna_2 - antenna_1;
    let mut antinodes = vec![antenna_1.clone(), antenna_2.clone()];

    let mut current = antenna_2.clone();
    loop {
        current = &current + &delta;
        if !is_within_bounds(&current, city_map) {
            break;
        }
        antinodes.push(current.clone());
    }

    let mut current = antenna_1.clone();
    loop {
        current = &current - &delta;
        if !is_within_bounds(&current, city_map) {
            break;
        }
        antinodes.push(current.clone());
    }

    antinodes
}

fn find_antinodes_for_freq(
    same_freq_antennas: &HashSet<Coord>,
    city_map: &CityMap,
    finder: AntinodesFinder,
) -> HashSet<Coord> {
    let mut antinodes_freq = HashSet::new();

    for coord1 in same_freq_antennas {
        for coord2 in same_freq_antennas {
            if coord1 != coord2 {
                let ans = finder(coord1, coord2, city_map);
                antinodes_freq.extend(ans);
            }
        }
    }

    antinodes_freq
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");
    let city_map = build_city_map(input);
    let antennas = find_antennas(&city_map);

    let antinodes_part1: HashSet<Coord> = antennas.values()
        .flat_map(|same_freq_antennas| find_antinodes_for_freq(same_freq_antennas, &city_map, find_antinodes_part1))
        .collect();

    let antinodes_part2: HashSet<Coord> = antennas.values()
        .flat_map(|same_freq_antennas| find_antinodes_for_freq(same_freq_antennas, &city_map, find_antinodes_part2))
        .collect();

    println!("Part 1: {}", antinodes_part1.len());
    println!("Part 1: {}", antinodes_part2.len());
}
