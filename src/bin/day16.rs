// Day 16: Reindeer Maze
// https://adventofcode.com/2024/day/16

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug)]
struct State {
    cost: u64,
    x: i32,
    y: i32,
    dir: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.x == other.x && self.y == other.y && self.dir == other.dir
    }
}
impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input: String = std::fs::read_to_string("inputs/day16.txt").unwrap();
    let (grid, start, end) = parse_input(&input);

    // Part one
    let lowest_score = find_lowest_score(&grid, &start, &end);
    println!("lowest_score={:?}", lowest_score);

    ()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let mut grid = Vec::with_capacity(140);
    let (mut start_row, mut start_col) = (0, 0);
    let (mut end_row, mut end_col) = (0, 0);

    let mut i = 0;
    for line in input.lines() {
        let mut row = Vec::with_capacity(140);
        let mut j = 0;
        for c in line.trim().chars() {
            if c == 'S' {
                (start_row, start_col) = (i, j);
            } else if c == 'E' {
                (end_row, end_col) = (i, j);
            }

            row.push(c);
            j += 1;
        }
        grid.push(row);
        i += 1;
    }

    (grid, (start_row, start_col), (end_row, end_col))
}

fn find_lowest_score(grid: &Vec<Vec<char>>, start: &(i32, i32), end: &(i32, i32)) -> u64 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);

    let mut dist = vec![vec![vec![u64::MAX; 4]; n as usize]; m as usize];
    dist[start.0 as usize][start.1 as usize][1] = 0;

    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        x: start.1,
        y: start.0,
        dir: 1,
    });

    while let Some(curr_state) = pq.pop() {
        if (curr_state.y, curr_state.x) == (end.0, end.1) {
            return curr_state.cost;
        }

        if curr_state.cost
            > dist[curr_state.y as usize][curr_state.x as usize][curr_state.dir as usize]
        {
            continue;
        }

        for inc_dir in -1..=1 {
            let next_dir = (curr_state.dir + inc_dir).rem_euclid(4);
            let next_cost = if inc_dir == 0 {
                let (inc_x, inc_y) = match next_dir {
                    0 => (0, -1),
                    1 => (1, 0),
                    2 => (0, 1),
                    3 => (-1, 0),
                    _ => unreachable!(),
                };

                let (next_x, next_y) = (curr_state.x + inc_x, curr_state.y + inc_y);

                if next_x < 0 || next_x >= n || next_y < 0 || next_y >= m {
                    continue;
                }

                if grid[next_y as usize][next_x as usize] == '#' {
                    continue;
                }

                curr_state.cost + 1
            } else {
                curr_state.cost + 1000
            };

            let (next_x, next_y) = if inc_dir == 0 {
                (
                    curr_state.x
                        + if next_dir == 1 {
                            1
                        } else if next_dir == 3 {
                            -1
                        } else {
                            0
                        },
                    curr_state.y
                        + if next_dir == 2 {
                            1
                        } else if next_dir == 0 {
                            -1
                        } else {
                            0
                        },
                )
            } else {
                (curr_state.x, curr_state.y)
            };

            if next_cost < dist[next_y as usize][next_x as usize][next_dir as usize] {
                dist[next_y as usize][next_x as usize][next_dir as usize] = next_cost;
                pq.push(State {
                    cost: next_cost,
                    x: next_x,
                    y: next_y,
                    dir: next_dir,
                });
            }
        }
    }

    std::u64::MAX
}

fn find_best_path_tiles(
    grid: &Vec<Vec<char>>,
    start: &(i32, i32),
    end: &(i32, i32),
    dist: &Vec<Vec<[u64; 4]>>,
    end_cost: u64,
) -> Vec<Vec<bool>> {
    todo!()
}
