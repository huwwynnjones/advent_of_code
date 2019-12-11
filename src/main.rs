mod manhatten;
mod signal_delay;

use crate::manhatten::load_path_directions_input;
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn main() {
    let modules_mass = match load_mass_input("modules_mass.txt") {
        Ok(m) => m,
        Err(err) => panic!("Unable to load the mass data: {}", err),
    };

    let total = total_fuel(&modules_mass);
    println!("The total fuel needed is {}", total);

    let program = match load_program_input("program.txt") {
        Ok(p) => p,
        Err(err) => panic!("Unable to load the program data: {}", err),
    };

    let executed_program = restore_gravity_assist(&program);
    println!(
        "The value of the output after restoring gravity assist is {}",
        executed_program[0]
    );
    match find_noun_and_verb(&program) {
        Some(answer) => {
            println!("Found a match for noun: {}, verb: {}", answer.0, answer.1);
            println!("100 * noun + verb = {}", (100 * answer.0) + answer.1)
        }
        None => println!("Nothing found"),
    }

    let wires = match load_path_directions_input("path_direction.txt") {
        Ok(w) => w,
        Err(err) => panic!("Unable to load the path direction data: {}", err),
    };
    let mut first_wire = Vec::new();
    for s in &wires.0 {
        first_wire.push(s.as_str())
    }
    let mut second_wire = Vec::new();
    for s in &wires.1 {
        second_wire.push(s.as_str())
    }
    println!(
        "The manhatten distance of the closest intersection is {}",
        manhatten::find_closest_intersection(&first_wire, &second_wire)
    );
    println!(
        "The steps distance of the closest intersection is {}",
        signal_delay::find_closest_intersection(&first_wire, &second_wire)
    );
}

fn find_noun_and_verb(program: &[i32]) -> Option<(i32, i32)> {
    let desired_output = 19_690_720;
    let original_instructions = Vec::from(program);
    for noun in 0..99 {
        for verb in 0..99 {
            let processed_instructions =
                process_instructions(Some(noun), Some(verb), &original_instructions);
            if processed_instructions[0] == desired_output {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn restore_gravity_assist(program: &[i32]) -> Vec<i32> {
    let memory = Vec::from(program);
    process_instructions(Some(12), Some(2), &memory)
}

const INSTRUCTION_LENGTH: usize = 4;

enum OpCode {
    Add,
    Multiply,
    Halt,
}

impl From<i32> for OpCode {
    fn from(opcode_number: i32) -> Self {
        match opcode_number {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn process_instructions(noun: Option<i32>, verb: Option<i32>, instructions: &[i32]) -> Vec<i32> {
    let mut processed_instructions = Vec::from(instructions);
    if let Some(n) = noun {
        processed_instructions[1] = n
    };
    if let Some(v) = verb {
        processed_instructions[2] = v
    };
    let mut instruction_pointer = 0;
    loop {
        match processed_instructions[instruction_pointer].into() {
            OpCode::Add => {
                update_instructions(&mut processed_instructions, instruction_pointer, |x, y| {
                    x + y
                })
            }
            OpCode::Multiply => {
                update_instructions(&mut processed_instructions, instruction_pointer, |x, y| {
                    x * y
                })
            }
            OpCode::Halt => break,
        }
        instruction_pointer += INSTRUCTION_LENGTH
    }
    processed_instructions
}

fn update_instructions(
    instructions: &mut [i32],
    instruction_pointer: usize,
    operation: fn(i32, i32) -> i32,
) {
    let positions = determine_positions(instruction_pointer, &instructions);
    instructions[positions.answer] = operation(
        instructions[positions.first_nmb],
        instructions[positions.second_nmb],
    )
}

struct Positions {
    first_nmb: usize,
    second_nmb: usize,
    answer: usize,
}

fn determine_positions(idx: usize, output_opcodes: &[i32]) -> Positions {
    let first_nmb = output_opcodes[idx + 1] as usize;
    let second_nmb = output_opcodes[idx + 2] as usize;
    let answer = output_opcodes[idx + 3] as usize;
    Positions {
        first_nmb,
        second_nmb,
        answer,
    }
}

fn calculate_fuel(mass: i32) -> i32 {
    let mut fuel = fuel_sub_calc(mass);
    if fuel == 0 {
        fuel
    } else {
        fuel = fuel + calculate_fuel(fuel);
        fuel
    }
}

fn fuel_sub_calc(mass: i32) -> i32 {
    let fuel = (mass as f32 / 3.0).floor() as i32;
    if (fuel - 2) < 0 {
        0
    } else {
        fuel - 2
    }
}

fn total_fuel(modules_mass: &[i32]) -> i32 {
    modules_mass.iter().map(|x| calculate_fuel(*x)).sum()
}

fn load_mass_input(file_name: &str) -> io::Result<Vec<i32>> {
    let modules_mass_input = File::open(file_name)?;
    let reader = BufReader::new(modules_mass_input);

    let mut modules_mass = Vec::new();

    reader.lines().for_each(|l| match l {
        Ok(mass) => match mass.parse::<i32>() {
            Ok(nmb) => modules_mass.push(nmb),
            Err(err) => panic!("{}", err),
        },
        Err(err) => panic!("{}", err),
    });

    Ok(modules_mass)
}

fn load_program_input(file_name: &str) -> io::Result<Vec<i32>> {
    let program_input = File::open(file_name)?;
    let mut reader = BufReader::new(program_input);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let program = buf
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|_| panic!("Failed to parse: {}", s))
        })
        .collect();
    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_opcodes() {
        let test_sets = [
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ];

        for test_set in test_sets.iter() {
            assert_eq!(process_instructions(None, None, &test_set.0), test_set.1)
        }
    }

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(2), 0);
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }

    #[test]
    fn test_total_fuel() {
        let modules_mass = [12, 14, 1969, 100756];
        assert_eq!(total_fuel(&modules_mass), (0 + 2 + 2 + 966 + 50346))
    }

    #[test]
    fn test_load_mass_input() {
        let correct_mass = [106947, 129138, 56893, 75116, 96763];
        assert_eq!(
            load_mass_input("modules_mass_test.txt").unwrap(),
            correct_mass
        );
    }

    #[test]
    fn test_load_program_input() {
        let correct_program = [1, 0, 0, 3, 1, 1, 2, 3, 0];
        assert_eq!(
            load_program_input("program_test.txt").unwrap(),
            correct_program
        );
    }
}
