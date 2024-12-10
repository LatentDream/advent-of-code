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
            nb_file: (input.len() / 2) as u32,
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
        let mut nb_file_to_check_remaining = self.nb_file;
        let mut compressed_storage = self.storage.clone();
        let mut f_end = self.storage.iter().rposition(|&x| x.is_some()).unwrap_or(0);
        let mut f_start = f_end;
        let target_file = self.storage[f_end].unwrap();
        while f_start > 0 && self.storage[f_start - 1] == Some(target_file) {
            f_start -= 1;
        }
        loop {
            let file_len = f_end - f_start + 1;

            let insert_at = compressed_storage
                .windows(file_len)
                .position(|window| window.iter().all(|&x| x.is_none()))
                .unwrap_or(compressed_storage.len());

            // Swap
            if insert_at + file_len <= f_start {
                compressed_storage[insert_at..insert_at + file_len]
                    .copy_from_slice(&self.storage[f_start..=f_end]);

                compressed_storage[f_start..=f_end]
                    .iter_mut()
                    .for_each(|x| *x = None);
            }

            // Find next file
            nb_file_to_check_remaining -= 1;
            if nb_file_to_check_remaining > 0 {
                f_end = self.storage[..f_start].iter().rposition(|&x| x.is_some()).unwrap_or(0);
                f_start = f_end;
                let target_file = self.storage[f_end].unwrap();
                while f_start > 0 && self.storage[f_start - 1] == Some(target_file) {
                    f_start -= 1;
                }
            } else {
                break;
            }
        }
        self.storage = compressed_storage;
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
