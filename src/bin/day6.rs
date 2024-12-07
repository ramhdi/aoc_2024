// Day 6: Guard Gallivant
// https://adventofcode.com/2024/day/6

use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;
    let (grid, start) = parse_input(&input);

    // Part one
    let visited_positions = count_visited_positions(grid.clone(), start);
    println!("visited_positions={:?}", visited_positions);

    // Part two
    let possible_obstructions = count_possible_obstructions(grid.clone(), start);
    println!("possible_obstructions={:?}", possible_obstructions);

    Ok(())
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (i32, i32)) {
    let mut grid = Vec::with_capacity(130);
    let (mut start_row, mut start_col) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(130);
        for (j, ch) in line.bytes().enumerate() {
            match ch {
                b'#' => row.push(255),
                b'^' => {
                    (start_row, start_col) = (i as i32, j as i32);
                    row.push(1)
                }
                _ => row.push(0),
            }
        }
        grid.push(row);
    }

    (grid, (start_row, start_col))
}

fn count_visited_positions(mut grid: Vec<Vec<u8>>, (sr, sc): (i32, i32)) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let (mut cr, mut cc) = (sr, sc);
    let mut count = 0;
    let directions = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut curr_direction = 0;

    while cr >= 0 && cc >= 0 && cr < m as i32 && cc < n as i32 {
        let (nr, nc) = (
            cr + directions[curr_direction].0,
            cc + directions[curr_direction].1,
        );

        if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
            break;
        }

        if grid[nr as usize][nc as usize] == 255 {
            curr_direction = (curr_direction + 1) % 4;
            continue;
        }

        grid[nr as usize][nc as usize] |= 1;
        (cr, cc) = (nr, nc);
    }

    for r in 0..m {
        for c in 0..n {
            if grid[r][c] == 1 {
                count += 1;
            }
        }
    }

    count
}

fn count_possible_obstructions(grid: Vec<Vec<u8>>, (sr, sc): (i32, i32)) -> usize {
    let m = grid.len();
    let n = grid[0].len();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn simulate_guard(grid: &Vec<Vec<u8>>, start: (i32, i32), directions: [(i32, i32); 4]) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited_states = HashSet::new();
        let mut cr = start.0;
        let mut cc = start.1;
        let mut dir = 0;

        while cr >= 0 && cc >= 0 && cr < m as i32 && cc < n as i32 {
            let state = (cr, cc, dir);
            if visited_states.contains(&state) {
                return true;
            }
            visited_states.insert(state);

            let (dr, dc) = directions[dir];
            let nr = cr + dr;
            let nc = cc + dc;

            if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                break;
            }

            if grid[nr as usize][nc as usize] == 255 {
                dir = (dir + 1) % 4;
                continue;
            }

            cr = nr;
            cc = nc;
        }

        false
    }

    let mut possible_obstructions = 0;

    for r in 0..m {
        for c in 0..n {
            if (r as i32, c as i32) == (sr, sc) || grid[r][c] == 255 {
                continue;
            }

            let mut modified_grid = grid.clone();
            modified_grid[r][c] = 255;

            if simulate_guard(&modified_grid, (sr, sc), directions) {
                possible_obstructions += 1;
            }
        }
    }

    possible_obstructions
}
