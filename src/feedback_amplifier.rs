use crate::diagnostic_program::{
    comparison, determine_positions, jump, process_opcode_and_param_mode, update_instructions,
    OpCode, ParameterMode, INPUT_OUTPUT_INS_LENGTH, INSTRUCTION_LENGTH,
};

#[derive(Debug)]
enum AmplifierState {
    Waiting,
    Halted,
}

#[derive(Debug)]
struct Amplifier {
    processed_instructions: Vec<i32>,
    instruction_pointer: usize,
    state: AmplifierState,
}

impl Amplifier {
    fn new(instructions: &[i32]) -> Amplifier {
        let processed_instructions = Vec::from(instructions);
        let instruction_pointer = 0;
        let state = AmplifierState::Waiting;
        Amplifier {
            processed_instructions,
            instruction_pointer,
            state,
        }
    }

    fn is_waiting(&self) -> bool {
        match self.state {
            AmplifierState::Halted => false,
            AmplifierState::Waiting => true,
        }
    }

    fn process_instructions(&mut self, input: &mut Vec<i32>) -> Vec<i32> {
        let mut output = Vec::new();

        loop {
            let opcode_mode = process_opcode_and_param_mode(
                self.processed_instructions[self.instruction_pointer],
            );
            let positions =
                determine_positions(self.instruction_pointer, &self.processed_instructions);
            match opcode_mode.opcode() {
                OpCode::Add => {
                    update_instructions(
                        &mut self.instruction_pointer,
                        &mut self.processed_instructions,
                        &positions,
                        &opcode_mode.parameter_modes(),
                        |x, y| x + y,
                    );
                }
                OpCode::Multiply => {
                    update_instructions(
                        &mut self.instruction_pointer,
                        &mut self.processed_instructions,
                        &positions,
                        &opcode_mode.parameter_modes(),
                        |x, y| x * y,
                    );
                }
                OpCode::Input => {
                    let first_param = positions
                        .first_param()
                        .expect("Expected to have the first parameter");
                    match input.pop() {
                        Some(i) => {
                            self.processed_instructions[first_param as usize] = i;
                            self.instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
                        }
                        None => break,
                    }
                }
                OpCode::Output => {
                    let first_param = positions
                        .first_param()
                        .expect("Expected to have the first parameter");
                    match opcode_mode.parameter_modes()[0] {
                        ParameterMode::Position => {
                            output.push(self.processed_instructions[first_param as usize])
                        }
                        ParameterMode::Immediate => output.push(first_param),
                    }
                    self.instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
                }
                OpCode::JumpIfTrue => jump(
                    positions.first_param(),
                    positions.second_param(),
                    &mut self.instruction_pointer,
                    &mut self.processed_instructions,
                    &opcode_mode.parameter_modes(),
                    |x| x != 0,
                ),
                OpCode::JumpIfFalse => jump(
                    positions.first_param(),
                    positions.second_param(),
                    &mut self.instruction_pointer,
                    &mut self.processed_instructions,
                    &opcode_mode.parameter_modes(),
                    |x| x == 0,
                ),
                OpCode::LessThan => {
                    comparison(
                        positions.first_param(),
                        positions.second_param(),
                        positions.answer(),
                        &mut self.processed_instructions,
                        &opcode_mode.parameter_modes(),
                        |x, y| x < y,
                    );
                    self.instruction_pointer += INSTRUCTION_LENGTH;
                }
                OpCode::Equals => {
                    comparison(
                        positions.first_param(),
                        positions.second_param(),
                        positions.answer(),
                        &mut self.processed_instructions,
                        &opcode_mode.parameter_modes(),
                        |x, y| x == y,
                    );
                    self.instruction_pointer += INSTRUCTION_LENGTH;
                }
                OpCode::Halt => {
                    self.state = AmplifierState::Halted;
                    break;
                }
            }
        }
        output
    }
}

fn process_feedback_phase_setting_sequence(sequence: &[i32], instructions: &[i32]) -> i32 {

    let mut amp_a = Amplifier::new(&instructions);
    let mut amp_b = Amplifier::new(&instructions);
    let mut amp_c = Amplifier::new(&instructions);
    let mut amp_d = Amplifier::new(&instructions);
    let mut amp_e = Amplifier::new(&instructions);
    //first run
    let mut out_a = amp_a.process_instructions(&mut vec![0, sequence[0]]);
    let mut out_b = amp_b.process_instructions(&mut vec![sequence[1]]);
    let mut out_c = amp_c.process_instructions(&mut vec![sequence[2]]);
    let mut out_d = amp_d.process_instructions(&mut vec![sequence[3]]);
    let mut out_e = amp_e.process_instructions(&mut vec![sequence[4]]);

    //run till all halted
    while amp_a.is_waiting()
        && amp_b.is_waiting()
        && amp_c.is_waiting()
        && amp_d.is_waiting()
        && amp_e.is_waiting()
    {
        out_a.append(&mut amp_a.process_instructions(&mut out_e));
        out_b.append(&mut amp_b.process_instructions(&mut out_a));
        out_c.append(&mut amp_c.process_instructions(&mut out_b));
        out_d.append(&mut amp_d.process_instructions(&mut out_c));
        out_e.append(&mut amp_e.process_instructions(&mut out_d));
    }

    *out_e.get(0).expect("There should be a final output")
}

pub(crate) fn find_best_feedback_phase_setting_sequence(instructions: &[i32]) -> i32 {
    let mut highest_output = 0;
    let start = 5;
    let end = 10;
    //this is a bit heavy handed i think
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
                            let output =
                                process_feedback_phase_setting_sequence(&sequence, instructions);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_feedback_phase_setting_sequence() {
        let test_set = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase_sequence = vec![9, 8, 7, 6, 5];
        assert_eq!(
            process_feedback_phase_setting_sequence(&phase_sequence, &test_set),
            139629729
        );

        let test_set = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phase_sequence = vec![9, 7, 8, 5, 6];
        assert_eq!(
            process_feedback_phase_setting_sequence(&phase_sequence, &test_set),
            18216
        )
    }

    #[test]
    fn test_find_best_feedback_phase_setting_sequence() {
        let test_set = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            find_best_feedback_phase_setting_sequence(&test_set),
            139629729
        );

        let test_set = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(find_best_feedback_phase_setting_sequence(&test_set), 18216)
    }
}
