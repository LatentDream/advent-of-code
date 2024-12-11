use aoc_2024::utils::{Coord, Grid, GridOps};
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

struct Map {
    topography: Grid<i32>,
    starts: Vec<Coord>,
    peaks: Vec<Coord>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let topography: Grid<i32> = input
            .split('\n')
            .filter(|l| !l.trim().is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).map(|d| d as i32).unwrap_or(-1))
                    .collect()
            })
            .collect();

        let mut starts = Vec::new();
        let mut peaks = Vec::new();

        for (y, row) in topography.iter().enumerate() {
            for (x, &alt) in row.iter().enumerate() {
                let coord = Coord::new(x as i32, y as i32);
                match alt {
                    0 => starts.push(coord),
                    9 => peaks.push(coord),
                    _ => (),
                }
            }
        }

        Map {
            topography,
            starts,
            peaks,
        }
    }

    fn get_height(&self, coord: &Coord) -> i32 {
        self.topography.get_at(coord).copied().unwrap_or(-1)
    }

    fn find_number_trail(self) -> u32 {
        let (x_max, y_max) = self.topography.get_dimensions();
        let mut accessibility_sets = Grid::create_empty(x_max, y_max, HashSet::new());

        for peak in &self.peaks {
            let mut peak_set = HashSet::new();
            peak_set.insert(peak.clone());
            accessibility_sets.set_at(peak, peak_set);
        }

        let mut queue = VecDeque::from(self.peaks.clone());
        let mut visited = HashSet::new();

        while let Some(current) = queue.pop_front() {
            // Get the set of peaks accessible from current position
            let current_peaks = accessibility_sets.get_at(&current).unwrap().clone();

            for adjacent in current.adjacents() {
                if let Some(adj_height) = self.get_valid_height(&adjacent) {
                    if self.get_height(&current) - 1 == adj_height {
                        let mut adjacent_peaks = accessibility_sets
                            .get_at(&adjacent)
                            .unwrap_or(&HashSet::new())
                            .clone();

                        let old_len = adjacent_peaks.len();
                        adjacent_peaks.extend(current_peaks.iter().cloned());

                        if adjacent_peaks.len() > old_len {
                            accessibility_sets.set_at(&adjacent, adjacent_peaks);
                            if visited.insert(adjacent.clone()) {
                                queue.push_back(adjacent);
                            }
                        }
                    }
                }
            }
        }

        self.print_computed_map(&accessibility_sets);

        self.starts
            .iter()
            .map(|start| {
                accessibility_sets
                    .get_at(start)
                    .unwrap_or(&HashSet::new())
                    .len() as u32
            })
            .sum()
    }

    fn get_valid_height(&self, pos: &Coord) -> Option<i32> {
        let height = self.get_height(pos);
        if height != -1 {
            Some(height)
        } else {
            None
        }
    }

    fn print_computed_map(&self, map: &Grid<HashSet<Coord>>) {
        println!("Computed Map:");
        for row in map {
            for cell in row {
                if cell.is_empty() {
                    print!("  .");
                } else {
                    print!("{:3}", cell.len());
                }
            }
            println!();
        }
        println!();
    }
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");
    let map = Map::new(&input);
    let number_of_trail = map.find_number_trail();
    println!("Part 1: {}", number_of_trail);
}
