use crate::manhatten::get_direction_and_length;

use std::collections::{HashMap, HashSet};

pub fn find_closest_intersection(first_wire: &[&str], second_wire: &[&str]) -> u32 {
    let first_wire_map = find_coordinates(&first_wire);
    let second_wire_map = find_coordinates(&second_wire);
    let intersections = find_intersections(&first_wire_map, &second_wire_map);
    find_closest(&intersections, &first_wire_map, &second_wire_map)
}

fn find_closest(
    intersections: &HashSet<(i32, i32)>,
    first_wire: &HashMap<(i32, i32), u32>,
    second_wire: &HashMap<(i32, i32), u32>,
) -> u32 {
    let mut distance = u32::max_value();
    for intersection in intersections {
        let current_distance =
            first_wire.get(&intersection).unwrap() + second_wire.get(&intersection).unwrap();
        if current_distance < distance {
            distance = current_distance
        }
    }
    distance
}

fn find_intersections(
    first_wire: &HashMap<(i32, i32), u32>,
    second_wire: &HashMap<(i32, i32), u32>,
) -> HashSet<(i32, i32)> {
    //collect two hashsets of co-ordinates then get the interseection of both
    let first_wire_coords: HashSet<(i32, i32)> = first_wire.keys().copied().collect();
    let second_wire_coords: HashSet<(i32, i32)> = second_wire.keys().copied().collect();
    first_wire_coords
        .intersection(&second_wire_coords)
        .copied()
        .collect()
}

fn find_coordinates(wire: &[&str]) -> HashMap<(i32, i32), u32> {
    let mut wire_coords = HashMap::new();
    let mut current_coord = (0, 0);
    let mut current_distance = 0;
    for path_instruction in wire {
        let direction_and_length = get_direction_and_length(path_instruction);
        let direction = direction_and_length.0;
        let length = direction_and_length.1;
        match direction {
            "U" => create_coords(
                length,
                &mut current_coord,
                &mut current_distance,
                &mut wire_coords,
                |x, y| (x, y + 1),
            ),
            "D" => create_coords(
                length,
                &mut current_coord,
                &mut current_distance,
                &mut wire_coords,
                |x, y| (x, y - 1),
            ),
            "L" => create_coords(
                length,
                &mut current_coord,
                &mut current_distance,
                &mut wire_coords,
                |x, y| (x - 1, y),
            ),
            "R" => create_coords(
                length,
                &mut current_coord,
                &mut current_distance,
                &mut wire_coords,
                |x, y| (x + 1, y),
            ),
            _ => panic!("Unknown direction {}", direction),
        }
    }
    wire_coords
}

pub fn create_coords(
    length: u32,
    current_coord: &mut (i32, i32),
    current_distance: &mut u32,
    coord_map: &mut HashMap<(i32, i32), u32>,
    operation: fn(i32, i32) -> (i32, i32),
) {
    for _ in 1..=length {
        *current_distance += 1;
        *current_coord = operation(current_coord.0, current_coord.1);
        coord_map.entry(*current_coord).or_insert(*current_distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_coordinates() {
        let first_wire = ["R8", "U5", "L5", "D3", "R5"];
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
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
        ];
        let mut correct_coords_map = HashMap::new();
        for (idx, coord) in correct_coords.iter().enumerate() {
            correct_coords_map.entry(*coord).or_insert((idx + 1) as u32);
        }
        let coords = find_coordinates(&first_wire);
        assert_eq!(coords, correct_coords_map);
    }

    #[test]
    fn test_find_intersections() {
        let first_wire = ["R8", "U5", "L5", "D3"];
        let second_wire = ["U7", "R6", "D4", "L4"];
        let mut correct_coords_set = HashSet::new();
        correct_coords_set.insert((3, 3));
        correct_coords_set.insert((6, 5));
        let first_wire_map = find_coordinates(&first_wire);
        let second_wire_map = find_coordinates(&second_wire);
        assert_eq!(
            find_intersections(&first_wire_map, &second_wire_map),
            correct_coords_set
        )
    }

    #[test]
    fn test_find_closest_intersection() {
        let first_wire_1 = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let second_wire_1 = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(
            find_closest_intersection(&first_wire_1, &second_wire_1),
            610
        );

        let first_wire_2 = [
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let second_wire_2 = [
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        assert_eq!(
            find_closest_intersection(&first_wire_2, &second_wire_2),
            410
        );
    }
}
