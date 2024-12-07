// Day 1: Historian Hysteria
// https://adventofcode.com/2024/day/1

use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day1.txt")?;
    let (left_list, right_list) = parse_input(&input);

    // Part one
    println!(
        "total_distance={:?}",
        total_distance(left_list.clone(), right_list.clone())
    );

    // Part two
    println!(
        "similarity_score={:?}",
        similarity_score(left_list.clone(), right_list.clone())
    );

    Ok(())
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut col1 = Vec::with_capacity(1000);
    let mut col2 = Vec::with_capacity(1000);

    for line in input.lines() {
        if let Some((left, right)) = line.split_once(char::is_whitespace) {
            col1.push(left.trim().parse::<i32>().unwrap_or(0));
            col2.push(right.trim().parse::<i32>().unwrap_or(0));
        }
    }

    (col1, col2)
}

fn total_distance(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i64 {
    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .fold(0, |mut acc, (&l, &r)| {
            acc += (l as i64 - r as i64).abs();
            acc
        })
}

fn similarity_score(left_list: Vec<i32>, right_list: Vec<i32>) -> u64 {
    let left_set: HashSet<i32> = left_list.into_iter().collect();
    let right_count: HashMap<i32, i32> =
        right_list
            .iter()
            .fold(HashMap::<i32, i32>::with_capacity(1000), |mut acc, &num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

    right_count.iter().fold(0, |mut acc, (&k, &v)| {
        if left_set.contains(&k) {
            acc += k as u64 * v as u64;
        }
        acc
    })
}
