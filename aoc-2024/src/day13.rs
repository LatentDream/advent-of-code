use std::{fmt, fs::read_to_string};

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");
    let mut systems = parse_system(&input);

    let mut nb_press_a = 0;
    let mut nb_press_b = 0;
    for system in systems.iter() {
        if let Some(sol) = system.solve() {
            if sol
                .data
                .iter()
                .all(|row| row.iter().all(|&x| (x.round() - x).abs() < 1e-10))
            {
                nb_press_a += sol.data[0][0].round() as i64;
                nb_press_b += sol.data[1][0].round() as i64;
            } else {
            }
        }
    }
    println!("Part 1: {}", nb_press_a * 3 + nb_press_b);

    let mut nb_press_a = 0;
    let mut nb_press_b = 0;
    for system in systems.iter_mut() {
        for row in system.b.data.iter_mut() {
            for element in row.iter_mut() {
                *element += 10000000000000.0;
            }
        }
        println!("{}", system);
        if let Some(sol) = system.solve() {
            println!("Solution {:?}", sol.data);
            if sol
                .data
                .iter()
                .all(|row| row.iter().all(|&x| (x.round() - x).abs() < 1e-3))
            {
                nb_press_a += sol.data[0][0].round() as i64;
                nb_press_b += sol.data[1][0].round() as i64;
                println!("Solvable")
            } else {
            }
            println!("");
        }
    }
    println!("Part 2: {}", nb_press_a * 3 + nb_press_b);
}

struct Matrix {
    nb_row: usize,
    nb_col: usize,
    pub data: Vec<Vec<f64>>,
}

struct SystemLinearEquations {
    a: Matrix,
    b: Matrix,
}

impl SystemLinearEquations {
    fn solve(&self) -> Option<Matrix> {
        // For numerical stability
        let (scaled_system, scale_factor) = self.scale_system();

        if let Some(inverted) = scaled_system.a.invert() {
            let mut solution = inverted.mul(&scaled_system.b)?;

            // Unscale
            for row in solution.data.iter_mut() {
                for element in row.iter_mut() {
                    *element *= scale_factor;
                }
            }
            return Some(solution);
        }
        None
    }

    fn scale_system(&self) -> (SystemLinearEquations, f64) {
        let max_b = self
            .b
            .data
            .iter()
            .flat_map(|row| row.iter())
            .map(|&x| x.abs())
            .fold(0.0, f64::max);

        let scale_factor = if max_b > 1e6 { max_b } else { 1.0 };
        let mut scaled_b = Matrix::new(self.b.data.clone());
        for row in scaled_b.data.iter_mut() {
            for element in row.iter_mut() {
                *element /= scale_factor;
            }
        }

        (
            SystemLinearEquations {
                a: Matrix::new(self.a.data.clone()),
                b: scaled_b,
            },
            scale_factor,
        )
    }
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let n = data.len();
        let m = data.get(0).map_or(0, |row| row.len());
        Matrix {
            nb_row: n,
            nb_col: m,
            data,
        }
    }

    fn invert(&self) -> Option<Matrix> {
        if self.nb_row != 2 || self.nb_col != 2 {
            return None;
        }

        let det = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        if det.abs() < 1e-10 {
            return None;
        }

        let inv_det = 1.0 / det;
        let inverted = vec![
            vec![self.data[1][1] * inv_det, -self.data[0][1] * inv_det],
            vec![-self.data[1][0] * inv_det, self.data[0][0] * inv_det],
        ];
        Some(Matrix::new(inverted))
    }

    fn mul(&self, other: &Matrix) -> Option<Matrix> {
        if self.nb_col != other.nb_row {
            return None;
        }

        let mut result = vec![vec![0.0; other.nb_col]; self.nb_row];
        for i in 0..self.nb_row {
            for j in 0..other.nb_col {
                for k in 0..self.nb_col {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Some(Matrix::new(result))
    }
}

// Parse Input

fn parse_button(line: &str) -> (f64, f64) {
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].split("+").nth(1).unwrap().parse().unwrap();
    let y = parts[1].split("+").nth(1).unwrap().parse().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (f64, f64) {
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].split("=").nth(1).unwrap().parse().unwrap();
    let y = parts[1].split("=").nth(1).unwrap().parse().unwrap();
    (x, y)
}

fn parse_system(input: &str) -> Vec<SystemLinearEquations> {
    let lines: Vec<&str> = input.lines().collect();
    let mut systems = Vec::with_capacity(lines.len() / 4);

    for chunk in lines.chunks(4) {
        let mut a_data = Vec::new();
        let mut b_data = Vec::new();
        let (a_x, a_y) = parse_button(chunk[0].split(": ").nth(1).unwrap());
        let (b_x, b_y) = parse_button(chunk[1].split(": ").nth(1).unwrap());
        let (prize_x, prize_y) = parse_prize(chunk[2].split(": ").nth(1).unwrap());

        a_data.push(vec![a_x, b_x]);
        a_data.push(vec![a_y, b_y]);
        b_data.push(vec![prize_x]);
        b_data.push(vec![prize_y]);

        systems.push(SystemLinearEquations {
            a: Matrix::new(a_data),
            b: Matrix::new(b_data),
        });
    }
    systems
}

// Helper function from Claude to print stuff:
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            write!(f, "[")?;
            for (j, &val) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, " ")?
                }
                write!(f, "{}", val)?;
            }
            write!(f, "]")?;
            if i < self.nb_row - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

impl fmt::Display for SystemLinearEquations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_rows = self.a.nb_row.max(self.b.nb_row);

        for i in 0..max_rows {
            if i < self.a.nb_row {
                write!(
                    f,
                    "{} ",
                    self.a.data[i]
                        .iter()
                        .map(|&x| format!("{:3}", x))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            } else {
                write!(f, "{}", " ".repeat(4 * self.a.nb_col))?;
            }

            write!(f, "      ")?;

            if i < self.b.nb_row {
                writeln!(
                    f,
                    "{}",
                    self.b.data[i]
                        .iter()
                        .map(|&x| format!("{:5}", x))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            } else {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
