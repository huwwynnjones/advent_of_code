const INSTRUCTION_LENGTH: usize = 4;
const INPUT_OUTPUT_INS_LENGTH: usize = 2;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
}

impl From<u32> for OpCode {
    fn from(opcode_number: u32) -> Self {
        match opcode_number {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            99 => OpCode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

impl From<i32> for OpCode {
    fn from(opcode_number: i32) -> Self {
        match opcode_number {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            99 => OpCode::Halt,
            x => panic!("Unknown opcode {}", x),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<u32> for ParameterMode {
    fn from(parameter_mode_number: u32) -> Self {
        match parameter_mode_number {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown parameter mode {}", parameter_mode_number),
        }
    }
}

pub(crate) fn process_instructions(input: Option<i32>, instructions: &[i32]) -> Vec<i32> {
    let mut processed_instructions = Vec::from(instructions);
    let mut output = Vec::new();
    let mut instruction_pointer = 0;

    loop {
        let opcode_mode =
            process_opcode_and_param_mode(processed_instructions[instruction_pointer]);
        match opcode_mode.opcode {
            OpCode::Add => {
                update_instructions(
                    &mut processed_instructions,
                    instruction_pointer,
                    &opcode_mode.parameter_modes,
                    |x, y| x + y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Multiply => {
                update_instructions(
                    &mut processed_instructions,
                    instruction_pointer,
                    &opcode_mode.parameter_modes,
                    |x, y| x * y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Input => {
                if let Some(i) = input {
                    let position = processed_instructions[instruction_pointer + 1] as usize;
                    processed_instructions[position] = i;
                };
                instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
            }
            OpCode::Output => {
                let position = processed_instructions[instruction_pointer + 1] as usize;
                match opcode_mode.parameter_modes[0] {
                    ParameterMode::Position => output.push(processed_instructions[position]),
                    ParameterMode::Immediate => output.push(position as i32),
                }
                instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
            }
            OpCode::Halt => break,
        }
    }
    output
}

fn process_opcode_and_param_mode(code: i32) -> OpcodeMode {
    let mut code_chars = Vec::new();
    for ch in code.to_string().chars() {
        code_chars.push(ch);
    }
    let mut first_two_chars: [Option<char>; 2] = [None, None];
    for item in &mut first_two_chars {
        *item = code_chars.pop();
    }
    let mut opcode = String::new();
    for idx in (0..2).rev() {
        if let Some(ch) = first_two_chars[idx] {
            opcode.push(ch)
        }
    }
    let mut parameter_modes = Vec::new();
    for _ in 0..3 {
        match code_chars.pop() {
            Some(ch) => {
                parameter_modes.push(ch.to_digit(10).expect("Unexpected parse failure {}").into());
            }
            None => parameter_modes.push(ParameterMode::Position),
        }
    }
    OpcodeMode {
        opcode: opcode
            .parse::<i32>()
            .expect("Unexpected parse failure")
            .into(),
        parameter_modes,
    }
}

#[derive(Debug)]
struct OpcodeMode {
    opcode: OpCode,
    parameter_modes: Vec<ParameterMode>,
}

fn update_instructions(
    instructions: &mut [i32],
    instruction_pointer: usize,
    parameter_modes: &[ParameterMode],
    operation: fn(i32, i32) -> i32,
) {
    let positions = determine_positions(instruction_pointer, &instructions);
    let first_nmb = match parameter_modes[0] {
        ParameterMode::Position => instructions[positions.first_nmb as usize],
        ParameterMode::Immediate => positions.first_nmb,
    };
    let second_nmb = match parameter_modes[1] {
        ParameterMode::Position => instructions[positions.second_nmb as usize],
        ParameterMode::Immediate => positions.second_nmb,
    };
    match parameter_modes[2] {
        ParameterMode::Position => {
            instructions[positions.answer as usize] = operation(first_nmb, second_nmb)
        }
        ParameterMode::Immediate => panic!("wtf i thought there were no immediates for writing"),
    }
}

struct Positions {
    first_nmb: i32,
    second_nmb: i32,
    answer: i32,
}

fn determine_positions(idx: usize, output_opcodes: &[i32]) -> Positions {
    let first_nmb = output_opcodes[idx + 1];
    let second_nmb = output_opcodes[idx + 2];
    let answer = output_opcodes[idx + 3];
    Positions {
        first_nmb,
        second_nmb,
        answer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_opcode_and_param_mode() {
        let input = 1002;
        let correct_output = OpcodeMode {
            opcode: OpCode::Multiply,
            parameter_modes: vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
            ],
        };
        let output = process_opcode_and_param_mode(input);
        assert_eq!(output.opcode, correct_output.opcode);
        assert_eq!(output.parameter_modes, correct_output.parameter_modes)
    }

    #[test]
    fn test_run_opcodes() {
        let test_sets = [
            (vec![3, 0, 4, 0, 99], vec![10]),
            (vec![3, 3, 1101, 100, -1, 1, 4, 1, 99], vec![9]),
        ];

        for test_set in test_sets.iter() {
            assert_eq!(process_instructions(Some(10), &test_set.0), test_set.1)
        }
    }
}
