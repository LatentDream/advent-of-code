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
    assert!(parts.len() == 2, "Invalid input format");

    let raw_rule = parts[0].to_string();
    let raw_input = parts[1].to_string();

    // Parse the rule
    let mut rules_priority: Rules = HashMap::new();
    for line in raw_rule.lines() {
        let [a, b]: [i32; 2] = line
            .split('|')
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid rule format: {}", line));
        rules_priority.entry(a).or_insert_with(Vec::new).push(b);
    }

    // Parse the input
    let inputs: Updates = raw_input
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().parse().unwrap()).collect())
        .collect();

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

    let dependencies_rules = get_depends_on_from_priority_rules(&rules_priority);
    (dependencies_rules, inputs)
}

pub fn sum_updates(updates: Updates) -> i32 {
    let mut total_valid = 0;
    for update in updates {
        assert!(
            update.len() % 2 == 1,
            "Is valid {:?}, but len is pair",
            update
        );
        total_valid += update[update.len() / 2];
    }
    total_valid
}

pub fn is_valid_update(update: &Update, dependency_rules: &Rules) -> bool {
    let mut seens: HashSet<i32> = HashSet::with_capacity(update.len());
    let mut need_update: HashSet<i32> = update.iter().cloned().collect();
    for page in update {
        // Check for dependencies
        assert!(!seens.contains(&page), "Cycle detected in problem");
        let page_deps = dependency_rules.get(&page);
        if let Some(deps) = page_deps {
            for dep in deps {
                if need_update.contains(&dep) {
                    return false;
                }
            }
        }
        seens.insert(*page);

        // Valid loop
        need_update.remove(page);
    }
    return true;
}

pub fn fix_update(update: &mut Update, rules_dependencies: &Rules) {
    let len = update.len();
    let set: HashSet<i32> = update.iter().cloned().collect();

    // Sort by number of dependencies
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

        a_deps.cmp(&b_deps)
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
}

pub fn solve() {
    let (dependency_rules, updates) = get_input();
    let (valid_updates, mut invalid_updates): (Updates, Updates) = updates
        .into_iter()
        .partition(|update| is_valid_update(update, &dependency_rules));

    // Sum valid input
    println!("Solution Part 1: {}", sum_updates(valid_updates));

    // Fix the invalid input
    for update in &mut invalid_updates {
        fix_update(update, &dependency_rules);
    }
    println!("Solution Part 1: {}", sum_updates(invalid_updates));
}
