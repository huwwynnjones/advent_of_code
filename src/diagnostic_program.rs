const INSTRUCTION_LENGTH: usize = 4;
const INPUT_OUTPUT_INS_LENGTH: usize = 2;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl From<u32> for OpCode {
    fn from(opcode_number: u32) -> Self {
        match opcode_number {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            x => panic!("Unknown opcode {}", x),
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
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
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
        let positions = determine_positions(instruction_pointer, &processed_instructions);
        match opcode_mode.opcode {
            OpCode::Add => {
                update_instructions(
                    &mut processed_instructions,
                    &positions,
                    &opcode_mode.parameter_modes,
                    |x, y| x + y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Multiply => {
                update_instructions(
                    &mut processed_instructions,
                    &positions,
                    &opcode_mode.parameter_modes,
                    |x, y| x * y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Input => {
                let first_param = positions
                    .first_param
                    .expect("Expected to have the first parameter");
                if let Some(i) = input {
                    processed_instructions[first_param as usize] = i;
                };
                instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
            }
            OpCode::Output => {
                let first_param = positions
                    .first_param
                    .expect("Expected to have the first parameter");
                match opcode_mode.parameter_modes[0] {
                    ParameterMode::Position => {
                        output.push(processed_instructions[first_param as usize])
                    }
                    ParameterMode::Immediate => output.push(first_param),
                }
                instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
            }
            OpCode::JumpIfTrue => jump(
                positions.first_param,
                positions.second_param,
                &mut instruction_pointer,
                &mut processed_instructions,
                &opcode_mode.parameter_modes,
                |x| x != 0,
            ),
            OpCode::JumpIfFalse => jump(
                positions.first_param,
                positions.second_param,
                &mut instruction_pointer,
                &mut processed_instructions,
                &opcode_mode.parameter_modes,
                |x| x == 0,
            ),
            OpCode::LessThan => {
                comparison(
                    positions.first_param,
                    positions.second_param,
                    positions.answer,
                    &mut processed_instructions,
                    &opcode_mode.parameter_modes,
                    |x, y| x < y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Equals => {
                comparison(
                    positions.first_param,
                    positions.second_param,
                    positions.answer,
                    &mut processed_instructions,
                    &opcode_mode.parameter_modes,
                    |x, y| x == y,
                );
                instruction_pointer += INSTRUCTION_LENGTH;
            }
            OpCode::Halt => break,
        }
    }
    output
}

fn jump(
    first_param: Option<i32>,
    second_param: Option<i32>,
    instruction_pointer: &mut usize,
    instructions: &mut [i32],
    parameter_modes: &[ParameterMode],
    operation: fn(i32) -> bool,
) {
    let first_param = first_param.expect("Expected to have the first parameter");
    let second_param = second_param.expect("Expected to have the second parameter");
    let comparison_result;
    match parameter_modes[0] {
        ParameterMode::Position => {
            comparison_result = operation(instructions[first_param as usize])
        }
        ParameterMode::Immediate => comparison_result = operation(first_param),
    }
    if comparison_result {
        match parameter_modes[1] {
            ParameterMode::Position => {
                *instruction_pointer = instructions[second_param as usize] as usize
            }
            ParameterMode::Immediate => *instruction_pointer = second_param as usize,
        }
    } else {
        *instruction_pointer += 3;
    }
}

fn comparison(
    first_param: Option<i32>,
    second_param: Option<i32>,
    answer: Option<i32>,
    instructions: &mut [i32],
    parameter_modes: &[ParameterMode],
    operation: fn(i32, i32) -> bool,
) {
    let first_param = first_param.expect("Expected to have the first parameter");
    let second_param = second_param.expect("Expected to have the second parameter");
    let answer = answer.expect("Expected to have the answer parameter");
    let comparison_result;
    match parameter_modes[0] {
        ParameterMode::Position => match parameter_modes[1] {
            ParameterMode::Position => {
                comparison_result = operation(
                    instructions[first_param as usize],
                    instructions[second_param as usize],
                )
            }
            ParameterMode::Immediate => {
                comparison_result = operation(instructions[first_param as usize], second_param)
            }
        },
        ParameterMode::Immediate => match parameter_modes[1] {
            ParameterMode::Position => {
                comparison_result = operation(first_param, instructions[second_param as usize])
            }
            ParameterMode::Immediate => comparison_result = operation(first_param, second_param),
        },
    }
    if comparison_result {
        instructions[answer as usize] = 1;
    } else {
        instructions[answer as usize] = 0;
    }
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
    positions: &Positions,
    parameter_modes: &[ParameterMode],
    operation: fn(i32, i32) -> i32,
) {
    let first_param = positions
        .first_param
        .expect("Expected to have the first parameter");
    let second_param = positions
        .second_param
        .expect("Expected to have the second parameter");
    let answer = positions
        .answer
        .expect("Expected to have the answer parameter");

    let first_nmb = match parameter_modes[0] {
        ParameterMode::Position => instructions[first_param as usize],
        ParameterMode::Immediate => first_param,
    };
    let second_nmb = match parameter_modes[1] {
        ParameterMode::Position => instructions[second_param as usize],
        ParameterMode::Immediate => second_param,
    };
    match parameter_modes[2] {
        ParameterMode::Position => instructions[answer as usize] = operation(first_nmb, second_nmb),
        ParameterMode::Immediate => panic!("wtf i thought there were no immediates for writing"),
    }
}

struct Positions {
    first_param: Option<i32>,
    second_param: Option<i32>,
    answer: Option<i32>,
}

fn determine_positions(instruction_pointer: usize, instructions: &[i32]) -> Positions {
    let ins_length = instructions.len();
    let first_param = match instruction_pointer + 1 {
        x if x < ins_length => Some(instructions[x]),
        _ => None,
    };
    let second_param = match instruction_pointer + 2 {
        x if x < ins_length => Some(instructions[x]),
        _ => None,
    };
    let answer = match instruction_pointer + 3 {
        x if x < ins_length => Some(instructions[x]),
        _ => None,
    };
    Positions {
        first_param,
        second_param,
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
        assert_eq!(output.parameter_modes, correct_output.parameter_modes);

        let input = 1105;
        let correct_output = OpcodeMode {
            opcode: OpCode::JumpIfTrue,
            parameter_modes: vec![
                ParameterMode::Immediate,
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

    #[test]
    fn test_conditional_opcodes() {
        let eq_to = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(process_instructions(Some(8), &eq_to), vec![1]);
        assert_eq!(process_instructions(Some(4), &eq_to), vec![0]);

        let less_than = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(process_instructions(Some(3), &less_than), vec![1]);
        assert_eq!(process_instructions(Some(10), &less_than), vec![0]);

        let eq_to = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(process_instructions(Some(8), &eq_to), vec![1]);
        assert_eq!(process_instructions(Some(4), &eq_to), vec![0]);

        let less_than = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(process_instructions(Some(3), &less_than), vec![1]);
        assert_eq!(process_instructions(Some(10), &less_than), vec![0]);
    }

    #[test]
    fn test_jump_opcodes() {
        let jump = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(process_instructions(Some(8), &jump), vec![1]);
        assert_eq!(process_instructions(Some(0), &jump), vec![0]);

        let jump = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(process_instructions(Some(8), &jump), vec![1]);
        assert_eq!(process_instructions(Some(0), &jump), vec![0]);
    }

    #[test]
    fn test_all_opcodes() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(process_instructions(Some(4), &prog), vec![999]);
        assert_eq!(process_instructions(Some(8), &prog), vec![1000]);
        assert_eq!(process_instructions(Some(11), &prog), vec![1001]);
    }
}
