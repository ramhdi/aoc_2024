// Day 19: Linen Layout
// https://adventofcode.com/2024/day/19

use std::collections::HashSet;

fn main() {
    let input: String = std::fs::read_to_string("inputs/day19.txt").unwrap();
    let (towels, designs) = parse_input(&input);
    // println!("{:?}", towels);
    // println!("{:?}", designs);

    // Part one
    let possible_designs = designs
        .iter()
        .map(|d| is_design_possible(d, &towels) as i32)
        .sum::<i32>();
    println!("{:?}", possible_designs);

    // Part two
    let possible_arrangements = designs
        .iter()
        .map(|d| possible_arrangements(d, &towels))
        .sum::<u64>();
    println!("{:?}", possible_arrangements);
}

fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let (mut towels, mut designs) = (HashSet::new(), Vec::new());
    let mut input_towels = true;
    for line in input.lines() {
        if line.trim().is_empty() {
            input_towels = false;
        } else {
            if input_towels {
                towels = line
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            } else {
                designs.push(line.trim().to_string());
            }
        }
    }

    (towels, designs)
}

fn is_design_possible(design: &String, towels: &HashSet<String>) -> bool {
    let n = design.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 0..n {
        if dp[i] {
            for towel in towels {
                if design[i..].starts_with(towel) {
                    dp[i + towel.len()] = true;
                }
            }
        }
    }

    dp[n]
}

fn possible_arrangements(design: &String, towels: &HashSet<String>) -> u64 {
    let n = design.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 0..n {
        if dp[i] > 0 {
            for towel in towels {
                if design[i..].starts_with(towel) {
                    dp[i + towel.len()] += dp[i];
                }
            }
        }
    }

    dp[n]
}
