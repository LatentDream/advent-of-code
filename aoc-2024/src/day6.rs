use std::{collections::HashSet, error::Error, fs, hash::{Hash, Hasher}, usize};

enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::RIGHT => (1, 0),
            Direction::LEFT => (-1, 0),
        }
    }
}

#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Coord {
    fn move_in_direction(&self, direction: &Direction) -> Option<Self> {
        let (dx, dy) = direction.delta();
        let new_x = self.x.checked_add_signed(dx)?;
        let new_y = self.y.checked_add_signed(dy)?;
        Some(Coord { x: new_x, y: new_y })
    }
}

impl Eq for Coord {}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

struct Guard {
    pub coord: Coord,
    pub direction: Direction,
    pub coord_visited: HashSet<Coord>
}

impl Guard {
    pub fn new(i: usize, j: usize, direction: Direction) -> Self {
        Guard {
            coord: Coord { x: i, y: j },
            direction,
            coord_visited: HashSet::new()
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    pub fn patrol(&mut self, obstacles: &[Coord]) -> Result<(), GameOver> {
        loop {
            let next_coord = self.coord.move_in_direction(&self.direction).ok_or(GameOver)?;
            if obstacles.contains(&next_coord) {
                self.turn();
                continue;
            }
            self.coord = next_coord.clone();
            self.coord_visited.insert(next_coord);
            break;
        }
        Ok(())
    }
}

struct Map {
    pub obstacle: Vec<Coord>,
    pub guard: Guard,
    pub map_size: (usize, usize)
}

// Top the dedication pour avoir le GameOver state Lol
#[derive(Debug)]
pub struct GameOver;
impl std::fmt::Display for GameOver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Game Over!")
    }
}
impl Error for GameOver {}

fn read_map() -> Map {
    let content = fs::read_to_string("input.txt").expect("the f*cking file to open");
    let location: Vec<Vec<char>> = content
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut obstacle = Vec::new();
    let mut guard: Option<Guard> = None;

    for (j, row) in location.iter().enumerate() {
        for (i, map_el) in row.iter().enumerate() {
            match *map_el {
                '#' => obstacle.push(Coord { x: i, y: j }),
                '^' => { guard = Some(Guard::new(i, j, Direction::UP)); },
                '>' => { guard = Some(Guard::new(i, j, Direction::RIGHT)); },
                'v' => { guard = Some(Guard::new(i, j, Direction::DOWN)); },
                '<' => { guard = Some(Guard::new(i, j, Direction::LEFT)); },
                _ => {}
            }
        }
    }

    Map {
        obstacle,
        guard: guard.expect("to a have guard in the map"),
        map_size: (location[0].len(), location.len()) 
    }
}


impl Map {

    pub fn update(&mut self) -> Result<(), GameOver> {
        self.guard.patrol(&self.obstacle)?;
        if self.guard.coord.x >= self.map_size.0 {
            return Err(GameOver)
        }
        if self.guard.coord.y >= self.map_size.1 {
            return Err(GameOver)
        }
        Ok(())
    }

    pub fn render(&self) {
        let (width, height) = self.map_size;
        for y in 0..height {
            for x in 0..width {
                let coord = Coord { x, y };
                if coord == self.guard.coord {
                    print!("{}", self.guard_char());
                } else if self.guard.coord_visited.contains(&coord) {
                    print!("x");
                } else if self.obstacle.contains(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn guard_char(&self) -> char {
        match self.guard.direction {
            Direction::UP => '^',
            Direction::DOWN => 'v',
            Direction::LEFT => '<',
            Direction::RIGHT => '>',
        }
    }

}

const RENDER: bool = true;

pub fn solve() {
    let mut map = read_map();

    let mut time: u64 = 0;
    loop {
        println!("Time: {}", time);
        if RENDER {
            map.render();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!();

        time += 1;
        if let Err(game_over) = map.update() {
            println!("Game Over: {}", game_over);
            break;
        }

    }

    println!("Part 1: {}", map.guard.coord_visited.len() - 1);
}
