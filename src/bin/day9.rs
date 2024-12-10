// Day 9: Disk Fragmenter
// https://adventofcode.com/2024/day/9

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("inputs/day9.txt")?;
    let disk_layout = parse_input(&input);

    // Part one
    let defrag_checksum = defrag_checksum(disk_layout.clone());
    println!("{:?}", defrag_checksum);

    // Part two
    let defrag_whole_checksum = defrag_whole_checksum(disk_layout.clone());
    println!("{:?}", defrag_whole_checksum);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut file_id = 0;
    for (i, len) in input.trim().bytes().enumerate() {
        let to_push = if i % 2 == 0 { Some(file_id) } else { None };
        for _ in 0..(len - b'0') {
            result.push(to_push);
        }
        if i % 2 == 0 {
            file_id += 1;
        }
    }

    result
}

fn defrag_checksum(mut disk_layout: Vec<Option<i32>>) -> i64 {
    let mut i = 0;
    let mut j = disk_layout.len() - 1;

    while i < j {
        while i < disk_layout.len() && disk_layout[i].is_some() {
            i += 1;
        }

        while j > 0 && disk_layout[j].is_none() {
            j -= 1;
        }

        disk_layout[i] = disk_layout[j];
        disk_layout[j] = None;

        i += 1;
        j -= 1;
    }

    disk_layout
        .iter()
        .enumerate()
        .map(|(idx, block)| match block {
            Some(file_id) => idx as i64 * *file_id as i64,
            None => 0,
        })
        .sum()
}

fn defrag_whole_checksum(mut disk_layout: Vec<Option<i32>>) -> i64 {
    let max_file_id = match disk_layout.iter().filter_map(|&x| x).max() {
        Some(m) => m,
        None => return 0,
    };

    for file_id in (0..=max_file_id).rev() {
        let file_positions: Vec<usize> = disk_layout
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == Some(file_id))
            .map(|(i, _)| i)
            .collect();

        if file_positions.is_empty() {
            continue;
        }

        let file_size = file_positions.len();
        let file_leftmost = file_positions[0];

        let mut candidate_start = None;
        let mut current_count = 0;
        let mut current_start = 0;

        for i in 0..file_leftmost {
            if disk_layout[i].is_none() {
                if current_count == 0 {
                    current_start = i;
                }
                current_count += 1;
                if current_count == file_size {
                    candidate_start = Some(current_start);
                    break;
                }
            } else {
                current_count = 0;
            }
        }

        if let Some(start_pos) = candidate_start {
            for &pos in &file_positions {
                disk_layout[pos] = None;
            }
            for offset in 0..file_size {
                disk_layout[start_pos + offset] = Some(file_id);
            }
        }
    }

    disk_layout
        .iter()
        .enumerate()
        .map(|(idx, block)| match block {
            Some(file_id) => idx as i64 * (*file_id as i64),
            None => 0,
        })
        .sum()
}

// fn print_disk_layout(disk_layout: &Vec<Option<i32>>) {
//     let to_print = disk_layout
//         .iter()
//         .map(|f| match f {
//             Some(num) => char::from_digit(*num as u32, 10).unwrap(),
//             None => '.',
//         })
//         .collect::<String>();

//     println!("{to_print}");
// }
