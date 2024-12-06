use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Rules = HashMap<i32, Vec<i32>>;
type Updates = Vec<Vec<i32>>;

fn get_input() -> (Rules, Updates) {
    let content = fs::read_to_string("input.txt").expect("the f*cking file to open");

    let parts: Vec<&str> = content.split("\n\n").collect();
    if parts.len() != 2 {
        panic!("Invalid input format");
    }

    let raw_rule = parts[0].to_string();
    let raw_input = parts[1].to_string();

    // Parse the rule
    let mut rules: Rules = HashMap::new();
    for line in raw_rule.lines() {
        let [a, b]: [i32; 2] = line
            .split('|')
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid rule format: {}", line));
        rules.entry(a).or_insert_with(Vec::new).push(b);
    }

    // Parse the input
    let inputs: Updates = raw_input
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().parse().unwrap()).collect())
        .collect();

    (rules, inputs)
}

pub fn inverse_rules(rules_priority: &Rules) -> Rules {
    let mut inverted_rules: Rules = HashMap::new();

    for (key, values) in rules_priority.iter() {
        for &value in values {
            inverted_rules
                .entry(value)
                .or_insert_with(Vec::new)
                .push(*key);
        }
    }

    inverted_rules
}

pub fn solve() {
    let (rules_priority, updates) = get_input();
    let rules_depends_on = inverse_rules(&rules_priority);

    let mut valid_updates: Updates = Vec::new();

    'update_loop: for update in updates {
        let mut seens: HashSet<i32> = HashSet::with_capacity(update.len());
        let mut need_update: HashSet<i32> = update.iter().cloned().collect();
        for page in &update {
            if seens.contains(&page) {
                panic!("Cycle :') detected in {:?}", update);
            }

            // Check for dependencies
            let page_deps = rules_depends_on.get(page);
            if let Some(deps) = page_deps {
                for dep in deps {
                    if need_update.contains(&dep) {
                        continue 'update_loop;
                    }
                }
            }
            seens.insert(*page);

            // Valid loop
            need_update.remove(page);
        }
        valid_updates.push(update);
    }
    println!("\n{:?}", valid_updates);

    let mut total_p1 = 0;
    for update in valid_updates {
        if update.len() % 2 != 1 {
            panic!("Is valid {:?}, but len is pair", update);
        }
        total_p1 += update[update.len() / 2];
    }

    println!("Solution {}", total_p1);
}
