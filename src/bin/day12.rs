use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Region {
    plant: char,
    area: u64,
    perimeter: u64,
    edges: Vec<((usize, usize), (usize, usize))>, // boundary edges as pairs of vertices
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("inputs/day12.txt")?;
    let farm = parse_input(&input);

    let regions = find_regions(&farm);

    // Part one: Calculate total fence price using perimeter
    let total_part_one: u64 = regions.iter().map(|r| r.area * r.perimeter).sum();
    println!("Part One Total Price (area*perimeter): {}", total_part_one);

    // Part two: Calculate total fence price using number of sides
    let total_part_two: u64 = regions
        .iter()
        .map(|r| {
            let sides = count_sides(&r.edges);
            r.area * sides
        })
        .sum();
    println!("Part Two Total Price (area*sides): {}", total_part_two);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut farm: Vec<Vec<char>> = Vec::new();
    for line in input.trim().lines() {
        let row: Vec<char> = line.chars().collect();
        farm.push(row);
    }
    farm
}

fn find_regions(farm: &Vec<Vec<char>>) -> Vec<Region> {
    let (m, n) = (farm.len(), farm[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut regions = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if !visited[i][j] {
                let mut region = Region {
                    plant: farm[i][j],
                    area: 0,
                    perimeter: 0,
                    edges: Vec::new(),
                };
                dfs(farm, &mut visited, &mut region, i, j);
                regions.push(region);
            }
        }
    }

    regions
}

/// DFS to identify a region and gather area, perimeter, and boundary edges.
fn dfs(
    farm: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    region: &mut Region,
    i: usize,
    j: usize,
) {
    let (m, n) = (farm.len(), farm[0].len());
    let mut stack = vec![(i, j)];
    visited[i][j] = true;

    while let Some((x, y)) = stack.pop() {
        region.area += 1;

        let directions = [
            (-1, 0), // up
            (1, 0),  // down
            (0, -1), // left
            (0, 1),  // right
        ];

        for &(dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                // Outside grid boundary
                region.perimeter += 1;
                record_edge(region, x, y, dx, dy);
            } else {
                let (nxu, nyu) = (nx as usize, ny as usize);
                if farm[nxu][nyu] == region.plant {
                    // Same plant, continue region
                    if !visited[nxu][nyu] {
                        visited[nxu][nyu] = true;
                        stack.push((nxu, nyu));
                    }
                } else {
                    // Different plant -> boundary edge
                    region.perimeter += 1;
                    record_edge(region, x, y, dx, dy);
                }
            }
        }
    }
}

/// Record the boundary edge of a cell. We represent vertices at cell corners:
/// Vertex coordinates correspond to grid intersections.
fn record_edge(region: &mut Region, x: usize, y: usize, dx: i32, dy: i32) {
    let edge = match (dx, dy) {
        (-1, 0) => ((x, y), (x, y + 1)),        // top
        (1, 0) => ((x + 1, y), (x + 1, y + 1)), // bottom
        (0, -1) => ((x, y), (x + 1, y)),        // left
        (0, 1) => ((x, y + 1), (x + 1, y + 1)), // right
        _ => panic!("Invalid direction"),
    };

    let edge_norm = if edge.0 <= edge.1 {
        edge
    } else {
        (edge.1, edge.0)
    };
    region.edges.push(edge_norm);
}

/// Count the number of sides for the given region's boundary edges.
fn count_sides(edges: &Vec<((usize, usize), (usize, usize))>) -> u64 {
    if edges.is_empty() {
        return 0;
    }

    // Build adjacency list
    let mut adjacency: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for &(v1, v2) in edges {
        adjacency.entry(v1).or_default().push(v2);
        adjacency.entry(v2).or_default().push(v1);
    }

    // Sort adjacency neighbors for deterministic traversal
    for (_k, nbrs) in adjacency.iter_mut() {
        nbrs.sort_unstable();
    }

    let mut visited_edges = HashSet::new();
    let mut total_sides = 0;

    // Find all cycles
    for &(v1, v2) in edges {
        if !visited_edges.contains(&normalize_edge((v1, v2))) {
            let cycle = find_cycle(&adjacency, v1, v2, &mut visited_edges);
            let sides = sides_in_cycle(&cycle);
            total_sides += sides;
        }
    }

    total_sides
}

/// Normalize an edge for visited checks
fn normalize_edge(e: ((usize, usize), (usize, usize))) -> ((usize, usize), (usize, usize)) {
    if e.0 <= e.1 {
        e
    } else {
        (e.1, e.0)
    }
}

/// Find a cycle starting from edge (v1->v2).
///
/// We walk until we return to v1. Because these are polygon boundaries,
/// we assume that eventually we return to the start forming a closed loop.
/// If a vertex has multiple neighbors, we pick the next unvisited edge different from `prev`.
///
/// If the input is a well-formed rectilinear polygon boundary, each edge set will form one or more closed loops.
fn find_cycle(
    adjacency: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    next: (usize, usize),
    visited_edges: &mut HashSet<((usize, usize), (usize, usize))>,
) -> Vec<(usize, usize)> {
    let mut cycle = vec![start];

    let mut current = next;
    let mut prev = start;

    visited_edges.insert(normalize_edge((start, next)));

    while current != start {
        cycle.push(current);

        // Select the next vertex
        let neighbors = &adjacency[&current];
        let mut found_next = None;
        for &nbr in neighbors {
            if nbr != prev {
                let edge = normalize_edge((current, nbr));
                if !visited_edges.contains(&edge) {
                    found_next = Some(nbr);
                    break;
                }
            }
        }

        let nxt = found_next.expect("No next vertex found. Inconsistent polygon?");
        visited_edges.insert(normalize_edge((current, nxt)));

        prev = current;
        current = nxt;
    }

    cycle
}

/// Count sides in a single polygon cycle. A side is a maximal straight run of edges in the same direction.
fn sides_in_cycle(cycle: &Vec<(usize, usize)>) -> u64 {
    if cycle.len() < 2 {
        return 0;
    }

    // Determine directions of edges
    let mut directions = Vec::new();
    for i in 0..cycle.len() {
        let v1 = cycle[i];
        let v2 = cycle[(i + 1) % cycle.len()];
        // Horizontal if row is same, vertical if col is same
        if v1.0 == v2.0 {
            directions.push('H');
        } else {
            directions.push('V');
        }
    }

    // Count how many segments of consecutive same-direction edges exist
    let mut sides = 1;
    for i in 1..directions.len() {
        if directions[i] != directions[i - 1] {
            sides += 1;
        }
    }
    sides
}
