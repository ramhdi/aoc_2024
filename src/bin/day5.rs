// Day 5: Print Queue
// https://adventofcode.com/2024/day/5

use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    let (rules, updates) = parse_input(&input);

    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|u| validate_update(u, &rules).is_some());

    // Part one
    let valid_middle_sum = valid_updates.iter().map(|u| u[u.len() / 2]).sum::<i32>();
    println!("{:?}", valid_middle_sum);

    // Part two
    let corrected_middle_sum = invalid_updates
        .iter()
        .map(|u| correct_order(u, &rules))
        .sum::<i32>();
    println!("{:?}", corrected_middle_sum);

    Ok(())
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (mut rules, mut updates) = (HashMap::with_capacity(1000), Vec::with_capacity(200));
    let mut input_rules = true;

    for line in input.lines() {
        if line.trim().is_empty() {
            input_rules = false;
            continue;
        }

        if input_rules {
            let (before, after) = line.split_once('|').unwrap();
            let before: i32 = before.trim().parse().unwrap();
            let after: i32 = after.trim().parse().unwrap();
            rules.entry(before).or_insert(HashSet::new()).insert(after);
        } else {
            updates.push(line.split(',').map(|e| e.trim().parse().unwrap()).collect());
        }
    }

    (rules, updates)
}

fn validate_update(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Option<i32> {
    let position: HashMap<i32, usize> = update.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    for (&before, after_pages) in rules.iter() {
        if let Some(&before_pos) = position.get(&before) {
            for &after in after_pages {
                if let Some(&after_pos) = position.get(&after) {
                    if before_pos > after_pos {
                        return None;
                    }
                }
            }
        }
    }

    Some(update[update.len() / 2])
}

fn correct_order(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut graph = HashMap::new();
    let mut in_degree = HashMap::new();

    let update_set: HashSet<_> = update.iter().cloned().collect();
    for &page in update_set.iter() {
        graph.entry(page).or_insert_with(Vec::new);
        in_degree.entry(page).or_insert(0);
    }

    for (&before, after_pages) in rules.iter() {
        if update_set.contains(&before) {
            for &after in after_pages {
                if update_set.contains(&after) {
                    graph.entry(before).or_insert_with(Vec::new).push(after);
                    *in_degree.entry(after).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    for (&node, &degree) in in_degree.iter() {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(neighbor_degree) = in_degree.get_mut(&neighbor) {
                    *neighbor_degree -= 1;
                    if *neighbor_degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted[sorted.len() / 2]
}
