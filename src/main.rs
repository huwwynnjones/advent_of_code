use std::{
    collections::HashSet,
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
        "The distance of the closest intersection is {}",
        find_closest_intersection(&first_wire, &second_wire)
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

fn find_closest_intersection(first_wire: &[&str], second_wire: &[&str]) -> i32 {
    let intersections = find_intersections(first_wire, second_wire);
    find_closest(intersections)
}

fn find_closest(intersections: HashSet<(i32, i32)>) -> i32 {
    let central_port = (0, 0);
    let mut distance;
    let mut iter = intersections.iter();
    match iter.next() {
        Some(intersection) => {
            distance = manhatten_distance(central_port, *intersection);
        }
        None => panic!("Empty set of intersections"),
    }

    for intersection in iter {
        let current_distance = manhatten_distance(central_port, *intersection);
        if current_distance < distance {
            distance = current_distance
        }
    }
    distance
}

fn manhatten_distance(central_port: (i32, i32), intersection: (i32, i32)) -> i32 {
    (intersection.0 - central_port.0).abs() + (intersection.1 - central_port.1).abs()
}

fn find_intersections(first_wire: &[&str], second_wire: &[&str]) -> HashSet<(i32, i32)> {
    //collect two hashsets of co-ordinates then get the interseection of both
    let first_wire_coords = find_coordinates(first_wire);
    let second_wire_coords = find_coordinates(second_wire);
    first_wire_coords
        .intersection(&second_wire_coords)
        .copied()
        .collect()
}

fn find_coordinates(wire: &[&str]) -> HashSet<(i32, i32)> {
    let mut wire_coords = HashSet::new();
    let mut current_coord = (0, 0);
    for path_instruction in wire {
        let direction_and_length = get_direction_and_length(path_instruction);
        let direction = direction_and_length.0;
        let length = direction_and_length.1;
        match direction {
            "U" => create_coords(length, &mut current_coord, &mut wire_coords, |x, y| {
                (x, y + 1)
            }),
            "D" => create_coords(length, &mut current_coord, &mut wire_coords, |x, y| {
                (x, y - 1)
            }),
            "L" => create_coords(length, &mut current_coord, &mut wire_coords, |x, y| {
                (x - 1, y)
            }),
            "R" => create_coords(length, &mut current_coord, &mut wire_coords, |x, y| {
                (x + 1, y)
            }),
            _ => panic!("Unknown direction {}", direction),
        }
    }
    wire_coords
}

fn create_coords(
    length: u32,
    current_coord: &mut (i32, i32),
    coord_set: &mut HashSet<(i32, i32)>,
    operation: fn(i32, i32) -> (i32, i32),
) {
    for _ in 1..=length {
        *current_coord = operation(current_coord.0, current_coord.1);
        coord_set.insert(*current_coord);
    }
}

fn get_direction_and_length(path_instruction: &str) -> (&str, u32) {
    let direction = match path_instruction.get(0..1) {
        Some(d) => d,
        None => panic!("Invalid path instruction {}", path_instruction),
    };
    let length = match path_instruction.get(1..) {
        Some(l) => match l.parse::<u32>() {
            Ok(nmb) => nmb,
            Err(err) => panic!("Could not parse {}: {}", l, err),
        },
        None => panic!("Invalid path instruction {}", path_instruction),
    };
    (direction, length)
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

fn load_path_directions_input(file_name: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let program_input = File::open(file_name)?;
    let mut reader = BufReader::new(program_input);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    let first_wire = split_and_collect(&buf);
    buf.clear();

    reader.read_line(&mut buf)?;
    let second_wire = split_and_collect(&buf);

    Ok((first_wire, second_wire))
}

fn split_and_collect(buffer: &str) -> Vec<String> {
    buffer
        .trim_end_matches('\n')
        .to_string()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhatten_distance() {
        assert_eq!(6, manhatten_distance((0, 0), (3, 3)));
        assert_eq!(6, manhatten_distance((0, 0), (-3, -3)));
    }

    #[test]
    fn test_find_coordinates() {
        let first_wire = ["R8", "U5", "L5", "D3"];
        let correct_coords: Vec<(i32, i32)> = vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (8, 1),
            (8, 2),
            (8, 3),
            (8, 4),
            (8, 5),
            (7, 5),
            (6, 5),
            (5, 5),
            (4, 5),
            (3, 5),
            (3, 4),
            (3, 3),
            (3, 2),
        ];
        let mut correct_coords_set = HashSet::new();
        for coord in correct_coords.iter() {
            correct_coords_set.insert(*coord);
        }
        let coords = find_coordinates(&first_wire);
        assert_eq!(coords, correct_coords_set);
    }

    #[test]
    fn test_find_intersections() {
        let first_wire = ["R8", "U5", "L5", "D3"];
        let second_wire = ["U7", "R6", "D4", "L4"];
        let mut correct_coords_set = HashSet::new();
        correct_coords_set.insert((3, 3));
        correct_coords_set.insert((6, 5));
        assert_eq!(
            find_intersections(&first_wire, &second_wire),
            correct_coords_set
        )
    }

    #[test]
    fn test_find_closest_intersection() {
        let first_wire_1 = ["R8", "U5", "L5", "D3"];
        let second_wire_1 = ["U7", "R6", "D4", "L4"];
        assert_eq!(find_closest_intersection(&first_wire_1, &second_wire_1), 6);

        let first_wire_3 = [
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let second_wire_3 = [
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        assert_eq!(
            find_closest_intersection(&first_wire_3, &second_wire_3),
            135
        );

        let first_wire_2 = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let second_wire_2 = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(
            find_closest_intersection(&first_wire_2, &second_wire_2),
            159
        );
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

    #[test]
    fn test_load_path_directions_input() {
        let correct_first_wire = vec!["R1000", "D722", "L887", "D371", "R430", "D952"];
        let correct_second_wire = vec!["L992", "D463", "R10", "D791", "R312", "D146"];
        let correct_pair = (
            array_to_string_vec(&correct_first_wire),
            array_to_string_vec(&correct_second_wire),
        );
        assert_eq!(
            load_path_directions_input("path_direction_test.txt").unwrap(),
            correct_pair
        );
    }

    fn array_to_string_vec(array: &[&str]) -> Vec<String> {
        array.iter().map(|s| s.to_string()).collect()
    }
}
