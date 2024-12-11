// Day 10: Hoof It
// https://adventofcode.com/2024/day/10

use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("inputs/day10.txt")?;
    let (trailheads, topo_map) = parse_input(&input);

    // Part one
    let trailhead_score = trailheads
        .iter()
        .map(|&(si, sj)| find_trailhead_score(si, sj, &topo_map).len() as i32)
        .sum::<i32>();
    println!("{:?}", trailhead_score);

    // Part two
    let trailhead_rating = trailheads
        .iter()
        .map(|&(si, sj)| find_trailhead_rating(si, sj, &topo_map))
        .sum::<i32>();
    println!("{:?}", trailhead_rating);

    Ok(())
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let topo_map: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|l| l.trim().bytes().map(|c| (c - b'0') as i32).collect())
        .collect();

    let trailheads = topo_map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &h)| match h {
                0 => Some((i as i32, j as i32)),
                _ => None,
            })
        })
        .collect();

    (trailheads, topo_map)
}

fn find_trailhead_score(i: i32, j: i32, map: &Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    match map[i as usize][j as usize] {
        9 => HashSet::from([(i, j)]),
        _ => next_neighbors(i, j, map)
            .map(|(ni, nj)| find_trailhead_score(ni, nj, map))
            .fold(HashSet::new(), |mut acc, set| {
                acc.extend(set);
                acc
            }),
    }
}

fn find_trailhead_rating(i: i32, j: i32, map: &Vec<Vec<i32>>) -> i32 {
    match map[i as usize][j as usize] {
        9 => 1,
        _ => next_neighbors(i, j, map)
            .map(|(ni, nj)| find_trailhead_rating(ni, nj, map))
            .sum(),
    }
}

fn next_neighbors<'a>(
    i: i32,
    j: i32,
    map: &'a Vec<Vec<i32>>,
) -> impl Iterator<Item = (i32, i32)> + 'a {
    let (m, n) = (map.len() as i32, map[0].len() as i32);
    let val = map[i as usize][j as usize];

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(move |(di, dj)| {
            let (ni, nj) = (i + di, j + dj);
            if ni >= 0 && ni < m && nj >= 0 && nj < n && map[ni as usize][nj as usize] == val + 1 {
                Some((ni, nj))
            } else {
                None
            }
        })
}
