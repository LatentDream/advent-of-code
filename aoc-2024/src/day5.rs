use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Rules = HashMap<i32, Vec<i32>>;
type Update = Vec<i32>;
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

pub fn get_depends_on_from_priority_rules(rules_priority: &Rules) -> Rules {
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

pub fn sum_updates(updates: Updates) -> i32 {
    let mut total_valid = 0;
    for update in updates {
        if update.len() % 2 != 1 {
            panic!("Is valid {:?}, but len is pair", update);
        }
        total_valid += update[update.len() / 2];
    }
    total_valid
}

pub fn fix_update(update: &mut Update, rules_dependencies: &Rules) {
    let len = update.len();
    println!("\ninput update {:?}", update);
    let set: HashSet<i32> = update.iter().cloned().collect();

    // Sort by number of dependencies and value (as tiebreaker)
    update.sort_by(|&a, &b| {
        let a_deps = rules_dependencies
            .get(&a)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|&&x| set.contains(&x))
            .count();
        let b_deps = rules_dependencies
            .get(&b)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|&&x| set.contains(&x))
            .count();

        // If same number of dependencies, sort by value
        if a_deps == b_deps {
            b.cmp(&a)
        } else {
            a_deps.cmp(&b_deps)
        }
    });

    // Bubble up elements that violate dependencies
    for i in 0..len {
        for j in (i + 1)..len {
            if let Some(deps) = rules_dependencies.get(&update[i]) {
                if deps.contains(&update[j]) && set.contains(&update[j]) {
                    update.swap(i, j);
                }
            }
        }
    }

    println!("Sorted update {:?}", update);
}

pub fn solve() {
    let (rules_priority, updates) = get_input();
    let rules_depends_on = get_depends_on_from_priority_rules(&rules_priority);

    let mut valid_updates: Updates = Vec::new();
    let mut invalid_updates: Updates = Vec::new();

    // Sort Valid | Invalid
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
                        invalid_updates.push(update);
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

    // Sum valid input
    println!("Solution Part 1: {}", sum_updates(valid_updates));

    // Fix the invalid input
    println!("Rules: {:?}", rules_priority);
    for update in &mut invalid_updates {
        fix_update(update, &rules_depends_on);
    }
    println!("Solution Part 1: {}", sum_updates(invalid_updates));
}
