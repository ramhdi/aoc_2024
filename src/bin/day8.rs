// Day 8: Resonant Collinearity
// https://adventofcode.com/2024/day/8

use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let (m, n, antenna_positions) = parse_input(&input);

    // Part one
    let count_antinodes = count_antinodes(m, n, &antenna_positions);
    println!("count_antinodes={:?}", count_antinodes);

    // Part two
    let count_repeating_antinodes = count_repeating_antinodes(m, n, &antenna_positions);
    println!("count_repeating_antinodes={:?}", count_repeating_antinodes);

    Ok(())
}

fn parse_input(input: &str) -> (i32, i32, HashMap<u8, HashSet<(i32, i32)>>) {
    let mut antenna_positions: HashMap<u8, HashSet<(i32, i32)>> = HashMap::new();
    let (mut m, mut n) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.bytes().enumerate() {
            if ch != b'.' {
                antenna_positions
                    .entry(ch)
                    .or_insert_with(HashSet::new)
                    .insert((i as i32, j as i32));
            }
            if m == 0 {
                n += 1;
            }
        }
        m += 1;
    }

    (m, n, antenna_positions)
}

fn count_antinodes(m: i32, n: i32, antenna_positions: &HashMap<u8, HashSet<(i32, i32)>>) -> usize {
    let mut antinode_set: HashSet<(i32, i32)> = HashSet::new();

    for positions in antenna_positions.values() {
        if positions.len() < 2 {
            continue;
        }

        let positions_vec: Vec<_> = positions.iter().cloned().collect();
        for (i, &p1) in positions_vec.iter().enumerate() {
            for &p2 in &positions_vec[i + 1..] {
                let d = (p2.0 - p1.0, p2.1 - p1.1);
                let n1 = (p1.0 - d.0, p1.1 - d.1);
                let n2 = (p2.0 + d.0, p2.1 + d.1);

                if n1.0 >= 0 && n1.0 < m && n1.1 >= 0 && n1.1 < n {
                    antinode_set.insert(n1);
                }

                if n2.0 >= 0 && n2.0 < m && n2.1 >= 0 && n2.1 < n {
                    antinode_set.insert(n2);
                }
            }
        }
    }

    antinode_set.len()
}

fn count_repeating_antinodes(
    m: i32,
    n: i32,
    antenna_positions: &HashMap<u8, HashSet<(i32, i32)>>,
) -> usize {
    let mut antinode_set: HashSet<(i32, i32)> = HashSet::new();

    for positions in antenna_positions.values() {
        if positions.len() < 2 {
            continue;
        }

        let positions_vec: Vec<_> = positions.iter().cloned().collect();
        for (i, &p1) in positions_vec.iter().enumerate() {
            for &p2 in &positions_vec[i + 1..] {
                antinode_set.insert(p1);
                antinode_set.insert(p2);

                let d = (p2.0 - p1.0, p2.1 - p1.1);
                let mut n1 = (p1.0 - d.0, p1.1 - d.1);
                while n1.0 >= 0 && n1.0 < m && n1.1 >= 0 && n1.1 < n {
                    antinode_set.insert(n1);
                    n1 = (n1.0 - d.0, n1.1 - d.1);
                }

                let mut n2 = (p2.0 + d.0, p2.1 + d.1);

                while n2.0 >= 0 && n2.0 < m && n2.1 >= 0 && n2.1 < n {
                    antinode_set.insert(n2);
                    n2 = (n2.0 + d.0, n2.1 + d.1);
                }
            }
        }
    }

    antinode_set.len()
}
