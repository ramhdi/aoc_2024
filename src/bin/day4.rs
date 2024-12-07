// Day 4: Ceres Search
// https://adventofcode.com/2024/day/4

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/day4.txt")?;
    let byte_matrix = parse_input(&input);
    let byte_horizontal = byte_matrix.clone();
    let byte_vertical = iter_vertical(&byte_matrix);
    let byte_diag1 = iter_diag1(&byte_matrix);
    let byte_diag2 = iter_diag2(&byte_matrix);

    // Part one
    let xmas_count = [byte_horizontal, byte_vertical, byte_diag1, byte_diag2]
        .iter()
        .map(|e| e.iter().map(count_xmas).sum::<usize>())
        .sum::<usize>();
    println!("xmas_count={:?}", xmas_count);

    // Part two
    let x_mas_count = window_2d_xmas(&byte_matrix);
    println!("x_mas_count={:?}", x_mas_count);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn count_xmas(input: &Vec<u8>) -> usize {
    input
        .windows(4)
        .filter(|w| (w == &[b'X', b'M', b'A', b'S']) || (w == &[b'S', b'A', b'M', b'X']))
        .count()
}

fn iter_vertical(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let (m, n) = (input.len(), input[0].len());
    let mut result = Vec::with_capacity(n);

    for j in 0..n {
        let mut entry = Vec::with_capacity(m);
        for i in 0..m {
            entry.push(input[i][j]);
        }
        result.push(entry);
    }

    result
}

fn iter_diag1(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let (m, n) = (input.len(), input[0].len());
    let mut result = Vec::with_capacity(m + n - 1);

    for start_col in 0..n {
        let mut entry = Vec::new();
        let mut i = 0;
        let mut j = start_col;
        while i < m && j < n {
            entry.push(input[i][j]);
            i += 1;
            j += 1;
        }
        result.push(entry);
    }

    for start_row in 1..m {
        let mut entry = Vec::new();
        let mut i = start_row;
        let mut j = 0;
        while i < m && j < n {
            entry.push(input[i][j]);
            i += 1;
            j += 1;
        }
        result.push(entry);
    }

    result
}

fn iter_diag2(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let (m, n) = (input.len(), input[0].len());
    let mut result = Vec::with_capacity(m + n - 1);

    for start_col in 0..n {
        let mut entry = Vec::new();
        let mut i = 0;
        let mut j = start_col;
        while i < m && j < n {
            entry.push(input[i][j]);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        result.push(entry);
    }

    for start_row in 1..m {
        let mut entry = Vec::new();
        let mut i = start_row;
        let mut j = n - 1;
        while i < m && j < n {
            entry.push(input[i][j]);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        result.push(entry);
    }

    result
}

fn window_2d_xmas(input: &[Vec<u8>]) -> usize {
    let (m, n) = (input.len(), input[0].len());
    let mut result = 0;

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let diag1 = &[input[i - 1][j - 1], input[i][j], input[i + 1][j + 1]];
            let diag2 = &[input[i + 1][j - 1], input[i][j], input[i - 1][j + 1]];

            if (diag1 == &[b'M', b'A', b'S'] || diag1 == &[b'S', b'A', b'M'])
                && (diag2 == &[b'M', b'A', b'S'] || diag2 == &[b'S', b'A', b'M'])
            {
                result += 1;
            }
        }
    }

    result
}
