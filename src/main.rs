mod amplifier;
mod diagnostic_program;
mod extra_secure_container;
mod feedback_amplifier;
mod intcode;
mod manhatten;
mod monitoring_station;
mod orbit;
mod program;
mod rocket_equation;
mod secure_container;
mod signal_delay;
mod space_image;

use crate::{
    amplifier::find_best_phase_setting_sequence,
    diagnostic_program::process_instructions,
    feedback_amplifier::find_best_feedback_phase_setting_sequence,
    intcode::IntcodeComputer,
    manhatten::load_path_directions_input,
    monitoring_station::{load_asteroid_input, AsteroidMap},
    orbit::{load_orbit_input, process_orbit_map},
    program::{find_noun_and_verb, load_program_input, restore_gravity_assist},
    rocket_equation::{load_mass_input, total_fuel},
    space_image::{
        create_final_image, create_layers_from_image_data, find_layer_with_lowest_nmb,
        load_image_data,
    },
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
    println!(
        "The mininum number of orbital transfers is {}",
        min_orbitals
    );

    let amplifier_program = match load_program_input("amplifier_program.txt") {
        Ok(p) => p,
        Err(err) => panic!("Unable to load the amplifier program data: {}", err),
    };

    let max_thruster = find_best_phase_setting_sequence(&amplifier_program);
    println!("The maximum thruster signal is {}", max_thruster);

    let max_feedback_thruster = find_best_feedback_phase_setting_sequence(&amplifier_program);
    println!(
        "The maximum feedback thruster signal is {}",
        max_feedback_thruster
    );

    let space_image_data = match load_image_data("image_data.txt") {
        Ok(d) => d,
        Err(err) => panic!("Unable to load the space image data: {}", err),
    };
    let layers = create_layers_from_image_data(&space_image_data, 25, 6);
    let layer = find_layer_with_lowest_nmb(&layers, 0).expect("There should be a layer found");
    let answer = layer.count_occurences_of(1) * layer.count_occurences_of(2);
    println!("The image corruption test answer is {}", answer);

    let final_image = create_final_image(&layers);
    println!("The final image is \n{}", final_image.data_as_message());

    let boost_program = match intcode::load_program_input("boost_program.txt") {
        Ok(p) => p,
        Err(err) => panic!("Unable to load the boost program data: {}", err),
    };

    let mut comp = IntcodeComputer::new(&boost_program);
    comp.run(&mut vec![1]);
    println!("The test run output is {:?}", comp.output());
    comp.load_new_instructions(&boost_program);
    comp.run(&mut vec![2]);
    println!("The boost run output is {:?}", comp.output());

    let asteroid_input = match load_asteroid_input("asteroid.txt") {
        Ok(m) => m,
        Err(err) => panic!("Unable to load the asteroid data: {}", err),
    };

    let astroid_map = AsteroidMap::new(&asteroid_input);
    let best_location = astroid_map.find_best_location();
    println!(
        "The best location for the monitoring station is ({}, {}), it can see {} asteroids",
        (best_location.0).0,
        (best_location.0).1,
        best_location.1
    );

    let asteroids = astroid_map.shoot_asteroids(best_location.0);
    let asteroid_200 = asteroids[199];
    println!(
        "The 200th asteroid calculation is {}",
        (asteroid_200.0 * 100) + asteroid_200.1
    )
}
