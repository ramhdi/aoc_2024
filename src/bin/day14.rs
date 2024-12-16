// Day 14: Restroom Redoubt
// https://adventofcode.com/2024/day/14

#[derive(Debug)]
struct Robot {
    ox: i32,
    oy: i32,
    vx: i32,
    vy: i32,
}

const LX: i32 = 101;
const LY: i32 = 103;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("inputs/day14.txt")?;
    let robots = parse_input(&input);

    // Part one
    let safety_after_100 = calculate_safety_factor(&robots, 100);
    println!("safety_after_100={safety_after_100}");

    // Part two
    let christmas_tree_time = find_christmas_tree_time(&robots);
    println!("christmas_tree_time={christmas_tree_time}");

    Ok(())
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.trim().split_once(' ').unwrap();
            let pos: Vec<&str> = pos.split(|c| c == '=' || c == ',').collect();
            let vel: Vec<&str> = vel.split(|c| c == '=' || c == ',').collect();
            Robot {
                ox: pos[1].parse().unwrap(),
                oy: pos[2].parse().unwrap(),
                vx: vel[1].parse().unwrap(),
                vy: vel[2].parse().unwrap(),
            }
        })
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], n: i32) -> i64 {
    let mut quadrant_counts = [0i64; 4];
    let midx = LX / 2;
    let midy = LY / 2;

    for robot in robots {
        let cx = (robot.ox + n * robot.vx).rem_euclid(LX);
        let cy = (robot.oy + n * robot.vy).rem_euclid(LY);

        if cx > midx && cy > midy {
            quadrant_counts[0] += 1;
        } else if cx > midx && cy < midy {
            quadrant_counts[1] += 1;
        } else if cx < midx && cy < midy {
            quadrant_counts[2] += 1;
        } else if cx < midx && cy > midy {
            quadrant_counts[3] += 1;
        }
    }

    quadrant_counts.iter().product()
}

fn find_christmas_tree_time(robots: &[Robot]) -> i32 {
    todo!()
}
