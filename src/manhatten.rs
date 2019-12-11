use std::{
    collections::HashSet,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

pub fn find_closest_intersection(first_wire: &[&str], second_wire: &[&str]) -> i32 {
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

pub fn find_coordinates(wire: &[&str]) -> HashSet<(i32, i32)> {
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

pub fn get_direction_and_length(path_instruction: &str) -> (&str, u32) {
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

pub fn load_path_directions_input(file_name: &str) -> io::Result<(Vec<String>, Vec<String>)> {
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
