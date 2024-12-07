// Day 7: Bridge Repair
// https://adventofcode.com/2024/day/7

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day7.txt")?;
    let calibrations = parse_input(&input);

    // Part one
    let sum_valid_calibration = calibrations
        .iter()
        .map(|(t, n)| valid_calibration(*t, n) as i64 * t)
        .sum::<i64>();
    println!("sum_valid_calibration={:?}", sum_valid_calibration);

    // Part two
    let sum_valid_calibration_2 = calibrations
        .iter()
        .map(|(t, n)| valid_calibration_2(*t, n) as i64 * t)
        .sum::<i64>();
    println!("sum_valid_calibration_2={:?}", sum_valid_calibration_2);

    Ok(())
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let mut calibrations = Vec::with_capacity(850);

    for line in input.lines() {
        let (target_str, nums_str) = line.split_once(':').unwrap();
        let target = target_str.trim().parse::<i64>().unwrap();
        let nums = nums_str
            .trim()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        calibrations.push((target, nums));
    }

    calibrations
}

fn valid_calibration(target: i64, nums: &[i64]) -> bool {
    let n = nums.len();
    match n {
        0 => false,
        1 => target == nums[0],
        _ => {
            valid_calibration(target - nums[n - 1], &nums[0..n - 1])
                || (target % nums[n - 1] == 0
                    && valid_calibration(target / nums[n - 1], &nums[0..n - 1]))
        }
    }
}

fn valid_calibration_2(target: i64, nums: &[i64]) -> bool {
    if nums.is_empty() {
        return false;
    }

    fn helper(target: i64, nums: &[i64], current_value: i64) -> bool {
        if nums.is_empty() {
            return current_value == target;
        }

        let next = nums[0];
        let rest = &nums[1..];

        if helper(target, rest, current_value + next) {
            return true;
        }

        if helper(target, rest, current_value * next) {
            return true;
        }

        if let Some(concatenated) = concat(current_value, next) {
            if helper(target, rest, concatenated) {
                return true;
            }
        }

        false
    }

    helper(target, &nums[1..], nums[0])
}

#[inline(always)]
fn concat(a: i64, b: i64) -> Option<i64> {
    format!("{}{}", a, b).parse::<i64>().ok()
}
