mod manhatten;
mod program;
mod rocket_equation;
mod signal_delay;

use crate::{
    manhatten::load_path_directions_input,
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
}
