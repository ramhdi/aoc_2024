// Day 18: RAM Run
// https://adventofcode.com/2024/day/18

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug)]
struct State {
    cost: u64,
    x: i32,
    y: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.x == other.x && self.y == other.y
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
    let input: String = std::fs::read_to_string("inputs/day18.txt").unwrap();
    let bytes = parse_input(&input);
    let (grid, start, end) = generate_grid(&bytes[0..1024]);

    // Part one
    let shortest_step = find_shortest_step(&grid, &start, &end);
    println!("shortest_step={:?}", shortest_step);

    // Part two
    let first_blocking_byte = find_first_blocking_byte(&bytes, &start, &end);
    println!("first_blocking_byte={:?}", first_blocking_byte);

    ()
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .into_iter()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect()
}

fn generate_grid(bytes: &[(usize, usize)]) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let mut grid = vec![vec!['.'; 71]; 71];

    for &(x, y) in bytes {
        grid[y][x] = '#';
    }

    (grid, (0, 0), (70, 70))
}

fn find_shortest_step(grid: &Vec<Vec<char>>, start: &(i32, i32), end: &(i32, i32)) -> Option<u64> {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);

    let mut dist = vec![vec![u64::MAX; n as usize]; m as usize];
    dist[start.0 as usize][start.1 as usize] = 0;

    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        x: start.1,
        y: start.0,
    });

    while let Some(curr_state) = pq.pop() {
        if (curr_state.y, curr_state.x) == (end.0, end.1) {
            return Some(curr_state.cost);
        }

        if curr_state.cost > dist[curr_state.y as usize][curr_state.x as usize] {
            continue;
        }

        for dir in 0..4 {
            let (inc_x, inc_y) = match dir {
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

            let next_cost = curr_state.cost + 1;

            if next_cost < dist[next_y as usize][next_x as usize] {
                dist[next_y as usize][next_x as usize] = next_cost;
                pq.push(State {
                    cost: next_cost,
                    x: next_x,
                    y: next_y,
                });
            }
        }
    }

    None
}

fn find_first_blocking_byte(
    bytes: &Vec<(usize, usize)>,
    start: &(i32, i32),
    end: &(i32, i32),
) -> Option<(usize, usize)> {
    let n = bytes.len();
    let mut k = 0;

    while k < n {
        match find_shortest_step(&generate_grid(&bytes[0..k]).0, start, end) {
            Some(_) => k += 1,
            None => {
                println!("{k}");
                return Some(bytes[k - 1]);
            }
        }
    }

    None
}
