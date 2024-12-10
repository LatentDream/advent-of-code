use std::{fmt, fs::read_to_string};

#[derive(Clone)]
struct Disk {
    storage: Vec<Option<i32>>,
    nb_file: u32,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.storage {
            match item {
                Some(num) => write!(f, "{}", num)?,
                None => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

fn find_contiguous_range(slice: &[Option<i32>], target: i32) -> (usize, usize) {
    return slice
        .iter()
        .rposition(|&x| x == Some(target))
        .map(|f_end| {
            let mut f_start = f_end;
            while f_start > 0 && slice[f_start - 1] == Some(target) {
                f_start -= 1;
            }
            (f_start, f_end)
        })
        .expect("The file to exist");
}

impl Disk {
    fn new(input: &String) -> Self {
        let storage = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_digit(10).expect("a valid digit") as usize)
            .enumerate()
            .flat_map(|(i, len)| {
                if i % 2 == 0 {
                    let current_file_id = i / 2;
                    vec![Some(current_file_id as i32); len]
                } else {
                    vec![None; len]
                }
            })
            .collect();
        Disk {
            storage,
            nb_file: (input.len() / 2) as u32 - 1,
        }
    }

    fn compress_in_fragment(&mut self) {
        let (mut ptr_l, mut ptr_r) = (0, self.storage.len() - 1);
        while ptr_l < ptr_r {
            // Find Empty space, and swap
            if self.storage[ptr_l].is_none() {
                // Find the ptr_r which is not None
                while self.storage[ptr_r].is_none() && ptr_l < ptr_r {
                    ptr_r -= 1;
                }
                // Do the Swap
                self.storage.swap(ptr_l, ptr_r);
                ptr_r -= 1;
            }
            ptr_l += 1;
        }
    }

    fn compress_in_contiguous(&mut self) {
        for target_file in (1..=self.nb_file).rev() {
            let (f_start, f_end) = find_contiguous_range(&self.storage, target_file as i32);
            let len = f_end - f_start + 1;

            // Find insert position
            let insert_at = self
                .storage
                .windows(len)
                .position(|window| window.iter().all(|&x| x.is_none()))
                .unwrap_or(self.storage.len());

            // Swap if it's a compression
            if insert_at + len <= f_start {
                for i in 0..len {
                    self.storage.swap(insert_at + i, f_start + i);
                }
            }
        }
    }

    fn checksum(&self) -> i64 {
        self.storage
            .iter()
            .enumerate()
            .map(|(idx, v)| idx as i64 * v.unwrap_or(0) as i64)
            .sum()
    }
}

pub fn solve() {
    let input = read_to_string("input.txt").expect("the file to open");

    let mut disk = Disk::new(&input);
    disk.compress_in_fragment();
    let checksum = disk.checksum();
    println!("Checksum {}", checksum);

    // Part 1
    let mut disk = Disk::new(&input);
    disk.compress_in_contiguous();
    let checksum = disk.checksum();

    println!("Checksum {}", checksum);
}
