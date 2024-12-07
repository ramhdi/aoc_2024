// Day 3: Mull It Over
// https://adventofcode.com/2024/day/3

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_MUL: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref RE_INSTRUCTIONS: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
}

enum Operation {
    Mul(i64, i64),
    Do,
    Dont,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;

    // Part one
    let valid_operations = get_valid_operations(&input);
    let sum_valid_results: i64 = valid_operations.iter().map(|(x, y)| x * y).sum();
    println!("sum_valid_results={:?}", sum_valid_results);

    // Part two
    let operations_with_do = get_valid_operations_with_do(&input);
    let sum_valid_results_with_do = operations_with_do
        .iter()
        .fold((0, true), |(sum, enabled), op| match op {
            Operation::Mul(x, y) if enabled => (sum + x * y, enabled),
            Operation::Mul(_, _) => (sum, enabled),
            Operation::Do => (sum, true),
            Operation::Dont => (sum, false),
        })
        .0;

    println!("sum_valid_results_with_do={:?}", sum_valid_results_with_do);
    Ok(())
}

fn get_valid_operations(input: &str) -> Vec<(i64, i64)> {
    RE_MUL
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i64>().unwrap();
            let y = cap[2].parse::<i64>().unwrap();
            (x, y)
        })
        .collect()
}

fn get_valid_operations_with_do(input: &str) -> Vec<Operation> {
    RE_INSTRUCTIONS
        .captures_iter(input)
        .filter_map(
            |cap| match (cap.get(1), cap.get(2), cap.get(0).map(|m| m.as_str())) {
                (Some(x_str), Some(y_str), _) => {
                    let x = x_str.as_str().parse::<i64>().unwrap();
                    let y = y_str.as_str().parse::<i64>().unwrap();
                    Some(Operation::Mul(x, y))
                }
                (_, _, Some("do()")) => Some(Operation::Do),
                (_, _, Some("don't()")) => Some(Operation::Dont),
                _ => None,
            },
        )
        .collect()
}
