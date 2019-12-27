mod diagnostic_program;
mod extra_secure_container;
mod manhatten;
mod orbit;
mod program;
mod rocket_equation;
mod secure_container;
mod signal_delay;

use crate::{
    diagnostic_program::process_instructions,
    manhatten::load_path_directions_input,
    orbit::{load_orbit_input, process_orbit_map},
    program::{find_noun_and_verb, load_program_input, restore_gravity_assist},
    rocket_equation::{load_mass_input, total_fuel},
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
    let start = 367_479;
    let end = 893_698;
    let count = (start..=end)
        .filter(|nmb| secure_container::password_check(*nmb))
        .count();
    println!("The number of different passwords for part 1 is {}", count);

    let count = (start..=end)
        .filter(|nmb| extra_secure_container::password_check(*nmb))
        .count();
    println!("The number of different passwords for part 2 is {}", count);

    let diagnostic_program = match load_program_input("diagnostic_program.txt") {
        Ok(p) => p,
        Err(err) => panic!("Unable to load the diagnostic program data: {}", err),
    };

    let diagnostic_output = process_instructions(Some(1), &diagnostic_program);
    println!(
        "The output for the diagnostic program is {:?}",
        &diagnostic_output
    );

    let radiator_diagnostic_output = process_instructions(Some(5), &diagnostic_program);
    println!(
        "The output for the thermal radiator diagnostic program is {:?}",
        &radiator_diagnostic_output
    );

    let orbit_map = match load_orbit_input("orbit_map.txt") {
        Ok(p) => p,
        Err(err) => panic!("Unable to load the orbit data: {}", err),
    };

    let mut orbit_map_ref = Vec::new();
    for s in &orbit_map {
        orbit_map_ref.push(s.as_str())
    }
    let com = process_orbit_map(&orbit_map_ref);
    let total_orbits = com.total_orbits();
    println!("The total number of orbits is {}", total_orbits);

    let min_orbitals = com.distance_crossing_common_point("YOU", "SAN");
    println!("The mininum number of orbital transfers is {}", min_orbitals)
}
