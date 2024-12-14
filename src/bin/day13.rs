// Day 13: Claw Contraption
// https://adventofcode.com/2024/day/13

#[derive(Debug)]
struct Claw {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    xt: i64,
    yt: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("inputs/day13.txt")?;
    let claws = parse_input(&input);
    // println!("{:?}", claws);

    // Part one
    let fewest_tokens = claws
        .iter()
        .map(|c| match play_claw(c) {
            Some((a, b)) => {
                // println!("a={a} b={b}");
                if a >= 0 && b >= 0 && a <= 100 && b <= 100 {
                    3 * a + b
                } else {
                    0
                }
            }
            None => 0,
        })
        .sum::<i64>();
    println!("fewest_token={}", fewest_tokens);

    // Part two
    let fewest_tokens_offset = claws
        .iter()
        .map(|c| Claw {
            x1: c.x1,
            y1: c.y1,
            x2: c.x2,
            y2: c.y2,
            xt: 10000000000000 + c.xt,
            yt: 10000000000000 + c.yt,
        })
        .map(|c| match play_claw(&c) {
            Some((a, b)) => {
                // println!("a={a} b={b}");
                if a >= 0 && b >= 0 {
                    3 * a + b
                } else {
                    0
                }
            }
            None => 0,
        })
        .sum::<i64>();
    println!("fewest_tokens_offset={}", fewest_tokens_offset);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Claw> {
    let mut claws = Vec::new();

    for block in input.split("\r\n\r\n") {
        let mut lines = block.lines();
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();

        let button_a_coords: Vec<i64> = button_a
            .split(|c| c == 'X' || c == 'Y' || c == '+' || c == ',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let (x1, y1) = (button_a_coords[0], button_a_coords[1]);

        let button_b_coords: Vec<i64> = button_b
            .split(|c| c == 'X' || c == 'Y' || c == '+' || c == ',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let (x2, y2) = (button_b_coords[0], button_b_coords[1]);

        let prize_coords: Vec<i64> = prize
            .split(|c| c == 'X' || c == 'Y' || c == '=' || c == ',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let (xt, yt) = (prize_coords[0], prize_coords[1]);

        claws.push(Claw {
            x1,
            y1,
            x2,
            y2,
            xt,
            yt,
        });
    }

    claws
}

fn play_claw(claw: &Claw) -> Option<(i64, i64)> {
    let d = claw.x1 * claw.y2 - claw.x2 * claw.y1;
    let da = claw.y2 * claw.xt - claw.x2 * claw.yt;
    let db = claw.x1 * claw.yt - claw.y1 * claw.xt;
    // println!("d={d} da={da} db={db}");

    match (da % d, db % d) {
        (0, 0) => Some((da / d, db / d)),
        _ => None,
    }
}
