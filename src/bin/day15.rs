// Day 15: Warehouse Woes
// https://adventofcode.com/2024/day/15

fn main() {
    let input: String = std::fs::read_to_string("inputs/day15.txt").unwrap();
    let (grid, moves, (start_row, start_col)) = parse_input(&input);

    // Part one
    let new_grid = execute_moves(&moves, &grid, &(start_row, start_col));
    let gps_sum = calculate_gps_sum(&new_grid);
    println!("gps_sum={:?}", gps_sum);

    // Part two
    let wide_grid = resize_grid(&grid);
    let new_wide_grid = execute_moves_wide(&moves, &wide_grid, &(start_row, start_col * 2));
    let wide_gps_sum = calculate_gps_sum_wide(&new_wide_grid);
    println!("wide_gps_sum={:?}", wide_gps_sum);

    ()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>, (i32, i32)) {
    let mut input_moves = false;
    let mut grid = Vec::with_capacity(50);
    let mut moves = Vec::with_capacity(20000);
    let (mut start_row, mut start_col) = (std::i32::MIN, std::i32::MIN);

    let mut i = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            input_moves = true;
        } else {
            if input_moves {
                moves.extend(line.trim().chars());
            } else {
                let mut row = Vec::with_capacity(50);
                let mut j = 0;
                for c in line.trim().chars() {
                    if c == '@' {
                        (start_row, start_col) = (i, j);
                    }

                    row.push(c);
                    j += 1;
                }
                grid.push(row);
                i += 1;
            }
        }
    }

    (grid, moves, (start_row, start_col))
}

fn execute_moves(
    moves: &Vec<char>,
    grid: &Vec<Vec<char>>,
    &(start_row, start_col): &(i32, i32),
) -> Vec<Vec<char>> {
    let mut grid = grid.clone();
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let mut curr_row = start_row;
    let mut curr_col = start_col;

    for &m in moves {
        let (dr, dc) = match m {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let nr = curr_row + dr;
        let nc = curr_col + dc;

        if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
            continue;
        }

        let next_cell = grid[nr as usize][nc as usize];

        match next_cell {
            '#' => {
                continue;
            }
            '.' => {
                grid[curr_row as usize][curr_col as usize] = '.';
                curr_row = nr;
                curr_col = nc;
                grid[curr_row as usize][curr_col as usize] = '@';
            }
            'O' => {
                let mut chain = vec![(nr, nc)];
                let mut cr = nr;
                let mut cc = nc;

                loop {
                    let rr = cr + dr;
                    let cc2 = cc + dc;
                    if rr < 0 || rr >= rows || cc2 < 0 || cc2 >= cols {
                        chain.clear();
                        break;
                    }

                    let cell = grid[rr as usize][cc2 as usize];
                    if cell == '.' {
                        chain.push((rr, cc2));
                        break;
                    } else if cell == 'O' {
                        chain.push((rr, cc2));
                        cr = rr;
                        cc = cc2;
                    } else {
                        chain.clear();
                        break;
                    }
                }

                if !chain.is_empty() && chain.len() > 1 {
                    let free_spot = chain.pop().unwrap();

                    grid[curr_row as usize][curr_col as usize] = '.';
                    grid[nr as usize][nc as usize] = '@';
                    curr_row = nr;
                    curr_col = nc;

                    chain.reverse();
                    let mut prev = free_spot;
                    for (br, bc) in chain {
                        grid[prev.0 as usize][prev.1 as usize] = 'O';
                        grid[br as usize][bc as usize] = '.';
                        prev = (br, bc);
                    }
                } else {
                    continue;
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    grid
}

fn calculate_gps_sum(grid: &Vec<Vec<char>>) -> i32 {
    let mut gps_sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                gps_sum += (100 * i + j) as i32;
            }
        }
    }

    gps_sum
}

fn resize_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    todo!()
}

fn execute_moves_wide(
    moves: &Vec<char>,
    grid: &Vec<Vec<char>>,
    &(start_row, start_col): &(i32, i32),
) -> Vec<Vec<char>> {
    todo!()
}

fn calculate_gps_sum_wide(grid: &Vec<Vec<char>>) -> i32 {
    todo!()
}
