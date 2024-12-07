// Day 2: Red-nosed Reports
// https://adventofcode.com/2024/day/2

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day2.txt")?;
    let reports = parse_input(&input);
    let _test = [
        [7, 6, 4, 2, 1],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ]
    .map(|e| e.to_vec())
    .to_vec();

    // Part one
    println!("safe_reports={:?}", safe_reports(&reports));

    // Part two
    println!(
        "safe_reports_dampened={:?}",
        safe_reports_dampened(&reports)
    );

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::with_capacity(1000);

    for line in input.lines() {
        reports.push(
            line.split(' ')
                .map(|num| num.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    reports
}

fn safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|levels| safe_levels(levels)).count()
}

fn safe_levels(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut diffs = levels.windows(2).map(|w| w[1] - w[0]);
    let first_diff = diffs.next().unwrap();

    if first_diff == 0 || first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }

    let direction = first_diff.signum();

    diffs.all(|diff| diff.signum() == direction && diff != 0 && diff.abs() >= 1 && diff.abs() <= 3)
}

fn safe_reports_dampened(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|levels| {
            (0..=levels.len()).any(|i| {
                let sub_levels: Vec<_> = levels
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &val)| if idx != i { Some(val) } else { None })
                    .collect();

                sub_levels.len() >= 2 && safe_levels(&sub_levels)
            })
        })
        .count()
}
