use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("inputs/day11.txt")?;
    let nums = parse_input(&input);

    // Part one
    let after_25_blinks = count_blink(&nums, 25);
    println!("{:?}", after_25_blinks);

    // Part two
    let after_75_blinks = count_blink(&nums, 75);
    println!("{:?}", after_75_blinks);

    Ok(())
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|c| c.parse::<u64>().unwrap())
        .collect()
}

fn count_blink(nums: &Vec<u64>, blinks: usize) -> usize {
    let mut stone_count: HashMap<u64, usize> = HashMap::new();

    for &num in nums {
        *stone_count.entry(num).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_stone_count: HashMap<u64, usize> = HashMap::new();

        for (&num, &count) in &stone_count {
            if num == 0 {
                *new_stone_count.entry(1).or_insert(0) += count;
            } else {
                let num_str = num.to_string();
                let n = num_str.len();
                if n % 2 == 0 {
                    let (left, right) = num_str.split_at(n / 2);
                    let (left, right) =
                        (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());
                    *new_stone_count.entry(left).or_insert(0) += count;
                    *new_stone_count.entry(right).or_insert(0) += count;
                } else {
                    let new_num = num * 2024;
                    *new_stone_count.entry(new_num).or_insert(0) += count;
                }
            }
        }

        stone_count = new_stone_count;
    }

    stone_count.values().sum()
}
