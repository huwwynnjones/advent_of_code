use crate::diagnostic_program::{
    comparison, determine_positions, jump, process_opcode_and_param_mode, update_instructions,
    OpCode, ParameterMode, INPUT_OUTPUT_INS_LENGTH, INSTRUCTION_LENGTH,
};

fn process_instructions(input: &mut Vec<i32>, instructions: &[i32]) -> Vec<i32> {
    let mut processed_instructions = Vec::from(instructions);
    let mut output = Vec::new();
    let mut instruction_pointer = 0;

    loop {
        let opcode_mode =
            process_opcode_and_param_mode(processed_instructions[instruction_pointer]);
        let positions = determine_positions(instruction_pointer, &processed_instructions);
        match opcode_mode.opcode() {
            OpCode::Add => {
                update_instructions(
                    &mut instruction_pointer,
                    &mut processed_instructions,
                    &positions,
                    &opcode_mode.parameter_modes(),
                    |x, y| x + y,
                );
            }
            OpCode::Multiply => {
                update_instructions(
                    &mut instruction_pointer,
                    &mut processed_instructions,
                    &positions,
                    &opcode_mode.parameter_modes(),
                    |x, y| x * y,
                );
            }
            OpCode::Input => {
                let first_param = positions
                    .first_param()
                    .expect("Expected to have the first parameter");
                if let Some(i) = input.pop() {
                    processed_instructions[first_param as usize] = i;
                };
                instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
            }
            OpCode::Output => {
                let first_param = positions
                    .first_param()
                    .expect("Expected to have the first parameter");
                match opcode_mode.parameter_modes()[0] {
                    ParameterMode::Position => {
                        output.push(processed_instructions[first_param as usize])
                    }
                    ParameterMode::Immediate => output.push(first_param),
                }
                instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
            }
            OpCode::JumpIfTrue => jump(
                positions.first_param(),
                positions.second_param(),
                &mut instruction_pointer,
                &mut processed_instructions,
                &opcode_mode.parameter_modes(),
                |x| x != 0,
            ),
            OpCode::JumpIfFalse => jump(
                positions.first_param(),
                positions.second_param(),
                &mut instruction_pointer,
                &mut processed_instructions,
                &opcode_mode.parameter_modes(),
                |x| x == 0,
            ),
            OpCode::LessThan => {
                comparison(
                    positions.first_param(),
                    positions.second_param(),
                    positions.answer(),
                    &mut processed_instructions,
                    &opcode_mode.parameter_modes(),
                    |x, y| x < y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Equals => {
                comparison(
                    positions.first_param(),
                    positions.second_param(),
                    positions.answer(),
                    &mut processed_instructions,
                    &opcode_mode.parameter_modes(),
                    |x, y| x == y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Halt => break,
        }
    }
    output
}

pub(crate) fn find_best_phase_setting_sequence(instructions: &[i32]) -> i32 {
    let mut highest_output = 0;
    let start = 0;
    let end = 5;

    for a in start..end {
        for b in start..end {
            for c in start..end {
                for d in start..end {
                    for e in start..end {
                        let sequence = vec![a, b, c, d, e];
                        let mut sorted_sequence = sequence.clone();
                        sorted_sequence.sort();
                        sorted_sequence.dedup();
                        if sorted_sequence.len() < sequence.len() {
                            continue;
                        } else {
                            let output = process_phase_setting_sequence(&sequence, instructions);
                            if output > highest_output {
                                highest_output = output
                            }
                        }
                    }
                }
            }
        }
    }
    highest_output
}

fn process_phase_setting_sequence(sequence: &[i32], instructions: &[i32]) -> i32 {
    //input in reverse order to be lazy about reversing
    let mut input_1 = vec![0, sequence[0]];
    let amp_1 = *process_instructions(&mut input_1, &instructions)
        .get(0)
        .expect("There should be an output");

    let mut input_2 = vec![amp_1, sequence[1]];
    let amp_2 = *process_instructions(&mut input_2, &instructions)
        .get(0)
        .expect("There should be an output");

    let mut input_3 = vec![amp_2, sequence[2]];
    let amp_3 = *process_instructions(&mut input_3, &instructions)
        .get(0)
        .expect("There should be an output");

    let mut input_4 = vec![amp_3, sequence[3]];
    let amp_4 = *process_instructions(&mut input_4, &instructions)
        .get(0)
        .expect("There should be an output");

    let mut input_5 = vec![amp_4, sequence[4]];
    *process_instructions(&mut input_5, &instructions)
        .get(0)
        .expect("There should be an output")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_phase_setting_sequence() {
        let phase_sequence = vec![4, 3, 2, 1, 0];
        let test_set = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(
            process_phase_setting_sequence(&phase_sequence, &test_set),
            43210
        );

        let phase_sequence = vec![0, 1, 2, 3, 4];
        let test_set = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(
            process_phase_setting_sequence(&phase_sequence, &test_set),
            54321
        );

        let phase_sequence = vec![1, 0, 4, 3, 2];
        let test_set = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(
            process_phase_setting_sequence(&phase_sequence, &test_set),
            65210
        );
    }

    #[test]
    fn test_find_best_phase_setting_sequence() {
        let test_set = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(find_best_phase_setting_sequence(&test_set), 43210);

        let test_set = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(find_best_phase_setting_sequence(&test_set), 54321);

        let test_set = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(find_best_phase_setting_sequence(&test_set), 65210);
    }
}
