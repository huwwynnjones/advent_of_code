use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

    let updated_program = restore_gravity_assist(&program);
    let executed_program = run_opcodes(&updated_program);
    println!("The value at position 0 is {}", executed_program[0])
}

fn restore_gravity_assist(program: &[i32]) -> Vec<i32> {
    let mut new_program = Vec::from(program);
    new_program[1] = 12;
    new_program[2] = 2;
    new_program
}

const OPCODE_LENGTH: usize = 4;

fn run_opcodes(input_opcodes: &[i32]) -> Vec<i32> {
    let mut output_opcodes = Vec::from(input_opcodes);
    let mut idx = 0;
    loop {
        match output_opcodes[idx] {
            1 => {
                let positions = determine_positions(idx, &output_opcodes);
                output_opcodes[positions.answer] =
                    output_opcodes[positions.first_nmb] + output_opcodes[positions.second_nmb]
            }
            2 => {
                let positions = determine_positions(idx, &output_opcodes);
                output_opcodes[positions.answer] =
                    output_opcodes[positions.first_nmb] * output_opcodes[positions.second_nmb]
            }
            99 => break,
            _ => panic!("Unknown opcode"),
        }
        idx += OPCODE_LENGTH
    }
    output_opcodes
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
        .map(|x| {
            x.parse::<i32>()
                .expect(format!("Failed to parse: {}", x).as_ref())
        })
        .collect();
    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_gravity_assist() {
        let correct_changes = [1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1];
        let mut input = [1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1];
        restore_gravity_assist(&mut input);
        assert_eq!(input, correct_changes);
    }

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
            assert_eq!(run_opcodes(&test_set.0), test_set.1)
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
