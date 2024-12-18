// Day 17: Chronospatial Computer
// https://adventofcode.com/2024/day/17

#[derive(Debug, Clone)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

fn main() {
    let input: String = std::fs::read_to_string("inputs/day17.txt").unwrap();
    let (initial_state, program) = parse_input(&input);

    // Part one
    let output = execute_program(initial_state.clone(), &program);
    println!(
        "output={:?}",
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // Part two
    // let lowest_a: u64 = 4294967295;
    // let mut lowest_a_state = initial_state.clone();
    // lowest_a_state.reg_a = lowest_a;
    // let lowest_a_output = execute_program(lowest_a_state, &program);
    // println!("lowest_a_output={:?}", lowest_a_output);

    let lowest_a = find_reg_a(initial_state.clone(), &program);
    println!("lowest_a={:?}", lowest_a);

    ()
}

fn parse_input(input: &str) -> (State, Vec<u64>) {
    let mut reg_a = 0u64;
    let mut reg_b = 0u64;
    let mut reg_c = 0u64;
    let mut program = Vec::new();

    for line in input.lines() {
        if line.starts_with("Register A:") {
            reg_a = line.split(": ").nth(1).unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Register B:") {
            reg_b = line.split(": ").nth(1).unwrap().parse::<u64>().unwrap();
        } else if line.starts_with("Register C:") {
            reg_c = line.split(": ").nth(1).unwrap().parse::<u64>().unwrap()
        } else if line.starts_with("Program:") {
            program = line
                .split(": ")
                .nth(1)
                .ok_or("Invalid Program format")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        }
    }

    (
        State {
            reg_a,
            reg_b,
            reg_c,
        },
        program,
    )
}

fn get_combo_operand(state: &State, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => state.reg_a,
        5 => state.reg_b,
        6 => state.reg_c,
        _ => unreachable!(),
    }
}

fn execute_program(mut state: State, program: &[u64]) -> Vec<u64> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match opcode {
            0 => {
                // adv: Divide A by 2^operand
                let denominator = 1 << get_combo_operand(&state, operand);
                state.reg_a /= denominator;
            }
            1 => {
                // bxl: B = B XOR literal operand
                state.reg_b ^= operand as u64;
            }
            2 => {
                // bst: B = combo operand % 8
                state.reg_b = get_combo_operand(&state, operand) % 8;
            }
            3 => {
                // jnz: If A != 0, jump to literal operand
                if state.reg_a != 0 {
                    instruction_pointer = operand as usize;
                    continue; // Skip increment of instruction pointer
                }
            }
            4 => {
                // bxc: B = B XOR C
                state.reg_b ^= state.reg_c;
            }
            5 => {
                // out: Output combo operand % 8
                output.push((get_combo_operand(&state, operand) % 8) as u64);
            }
            6 => {
                // bdv: Divide A by 2^operand and store in B
                let denominator = 1 << get_combo_operand(&state, operand);
                state.reg_b = state.reg_a / denominator;
            }
            7 => {
                // cdv: Divide A by 2^operand and store in C
                let denominator = 1 << get_combo_operand(&state, operand);
                state.reg_c = state.reg_a / denominator;
            }
            _ => panic!("Invalid opcode encountered!"),
        }

        instruction_pointer += 2;
    }

    output
}

fn find_reg_a(mut curr_state: State, program: &[u64]) -> u64 {
    let mut a = 4294967295u64;
    while a < std::u64::MAX {
        curr_state.reg_a = a;
        let output = execute_program(curr_state.clone(), program);
        // println!("output={:?} program={:?}", output, program);

        if &output == program {
            return a;
        }

        a += 1;
    }

    std::u64::MAX
}
