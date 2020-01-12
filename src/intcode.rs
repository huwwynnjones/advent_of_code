use std::{
    collections::HashMap,
    convert::TryInto,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

const INSTRUCTION_LENGTH: u64 = 4;
const INPUT_OUTPUT_INS_LENGTH: u64 = 2;

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
    AdjustRelativeBaseOffset,
}

impl From<u64> for OpCode {
    fn from(opcode_number: u64) -> Self {
        match opcode_number {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustRelativeBaseOffset,
            99 => OpCode::Halt,
            x => panic!("Unknown opcode {}", x),
        }
    }
}

impl From<i64> for OpCode {
    fn from(opcode_number: i64) -> Self {
        match opcode_number {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustRelativeBaseOffset,
            99 => OpCode::Halt,
            x => panic!("Unknown opcode {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<u64> for ParameterMode {
    fn from(parameter_mode_number: u64) -> Self {
        match parameter_mode_number {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode {}", parameter_mode_number),
        }
    }
}

impl From<u32> for ParameterMode {
    fn from(parameter_mode_number: u32) -> Self {
        match parameter_mode_number {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode {}", parameter_mode_number),
        }
    }
}

#[derive(Debug)]
enum ComputerState {
    Running,
    Waiting,
    Halted,
}

pub struct IntcodeComputer {
    memory: HashMap<u64, i64>,
    output: Vec<i64>,
    instruction_pointer: u64,
    relative_base_offset: u64,
    state: ComputerState,
}

impl IntcodeComputer {
    pub fn new(instructions: &[i64]) -> IntcodeComputer {
        let mut memory = HashMap::new();
        for (idx, i) in instructions.iter().enumerate() {
            memory.insert(idx as u64, *i);
        }
        IntcodeComputer {
            memory,
            output: Vec::new(),
            instruction_pointer: 0,
            relative_base_offset: 0,
            state: ComputerState::Halted,
        }
    }

    fn is_waiting(&self) -> bool {
        match self.state {
            ComputerState::Waiting => true,
            _ => false,
        }
    }

    pub fn output(&self) -> &Vec<i64> {
        &self.output
    }

    pub fn load_new_instructions(&mut self, instructions: &[i64]) {
        self.memory.clear();
        for (idx, i) in instructions.iter().enumerate() {
            self.memory.insert(idx as u64, *i);
        }
        self.output.clear();
        self.instruction_pointer = 0;
        self.relative_base_offset = 0;
        self.state = ComputerState::Halted
    }

    pub fn run(&mut self, input: &mut Vec<i64>) {
        self.state = ComputerState::Running;
        loop {
            let opcode_mode = process_opcode_and_param_mode(memory_access(
                &mut self.memory,
                self.instruction_pointer as i64,
                0,
            ));
            let positions = determine_positions(self.instruction_pointer, &mut self.memory);
            match opcode_mode.opcode() {
                OpCode::Add => {
                    run_instructions(
                        &mut self.instruction_pointer,
                        &mut self.memory,
                        self.relative_base_offset,
                        &positions,
                        &opcode_mode,
                        |x, y| x + y,
                    );
                }
                OpCode::Multiply => {
                    run_instructions(
                        &mut self.instruction_pointer,
                        &mut self.memory,
                        self.relative_base_offset,
                        &positions,
                        &opcode_mode,
                        |x, y| x * y,
                    );
                }
                OpCode::Input => {
                    let first_param = positions
                        .first_param()
                        .expect("Expected to have the first parameter");
                    match input.pop() {
                        Some(i) => {
                            match opcode_mode.first_parameter_mode() {
                                ParameterMode::Position | ParameterMode::Immediate => {
                                    memory_update(&mut self.memory, first_param, 0, i);
                                }
                                ParameterMode::Relative => {
                                    memory_update(
                                        &mut self.memory,
                                        first_param,
                                        self.relative_base_offset,
                                        i,
                                    )
                                }
                            }
                            self.instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
                        }
                        None => {
                            self.state = ComputerState::Waiting;
                            break
                        }
                    }
                }
                OpCode::Output => {
                    let first_param = positions
                        .first_param()
                        .expect("Expected to have the first parameter");
                    match opcode_mode.first_parameter_mode() {
                        ParameterMode::Position => {
                            self.output
                                .push(memory_access(&mut self.memory, first_param, 0))
                        }
                        ParameterMode::Immediate => self.output.push(first_param),
                        ParameterMode::Relative => self.output.push(memory_access(
                            &mut self.memory,
                            first_param,
                            self.relative_base_offset,
                        )),
                    }
                    self.instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
                }
                OpCode::JumpIfTrue => jump(
                    positions.first_param(),
                    positions.second_param(),
                    &mut self.instruction_pointer,
                    self.relative_base_offset,
                    &mut self.memory,
                    &opcode_mode,
                    |x| x != 0,
                ),
                OpCode::JumpIfFalse => jump(
                    positions.first_param(),
                    positions.second_param(),
                    &mut self.instruction_pointer,
                    self.relative_base_offset,
                    &mut self.memory,
                    &opcode_mode,
                    |x| x == 0,
                ),
                OpCode::LessThan => {
                    comparison(
                        positions.first_param(),
                        positions.second_param(),
                        positions.answer(),
                        self.relative_base_offset,
                        &mut self.memory,
                        &opcode_mode,
                        |x, y| x < y,
                    );
                    self.instruction_pointer += INSTRUCTION_LENGTH;
                }
                OpCode::Equals => {
                    comparison(
                        positions.first_param(),
                        positions.second_param(),
                        positions.answer(),
                        self.relative_base_offset,
                        &mut self.memory,
                        &opcode_mode,
                        |x, y| x == y,
                    );
                    self.instruction_pointer += INSTRUCTION_LENGTH;
                }
                OpCode::AdjustRelativeBaseOffset => {
                    let first_param = positions
                        .first_param()
                        .expect("Expected to have the first parameter");
                    match opcode_mode.first_parameter_mode() {
                        ParameterMode::Position => {
                            self.relative_base_offset = adjusted_relative_base_offset(
                                self.relative_base_offset,
                                memory_access(&mut self.memory, first_param, 0),
                            )
                        }
                        ParameterMode::Immediate => {
                            self.relative_base_offset = adjusted_relative_base_offset(
                                self.relative_base_offset,
                                first_param,
                            )
                        }
                        ParameterMode::Relative => {
                            self.relative_base_offset = adjusted_relative_base_offset(
                                self.relative_base_offset,
                                memory_access(
                                    &mut self.memory,
                                    first_param,
                                    self.relative_base_offset,
                                ),
                            )
                        }
                    }
                    self.instruction_pointer += INPUT_OUTPUT_INS_LENGTH;
                }
                OpCode::Halt => {
                    self.state = ComputerState::Halted;
                    break;
                }
            }
        }
    }
}

fn memory_access(memory: &mut HashMap<u64, i64>, location_value: i64, offset: u64) -> i64 {
    let location = convert_to_location(location_value, offset);
    *memory.entry(location).or_insert(0)
}

fn memory_update(memory: &mut HashMap<u64, i64>, location_value: i64, offset: u64, value: i64) {
    let location = convert_to_location(location_value, offset);
    memory.insert(location, value);
}

fn convert_to_location(value: i64, offset: u64) -> u64 {
    let location = value + offset as i64;
    if location.is_negative() {
        panic!("Negative locations should not occur")
    } else {
        location as u64
    }
}

fn adjusted_relative_base_offset(current_base_offset: u64, adjustment: i64) -> u64 {
    if adjustment.is_negative() {
        current_base_offset.saturating_sub(adjustment.abs() as u64)
    } else {
        current_base_offset + adjustment as u64
    }
}

fn jump(
    first_param: Option<i64>,
    second_param: Option<i64>,
    instruction_pointer: &mut u64,
    relative_base_offset: u64,
    memory: &mut HashMap<u64, i64>,
    opcode_mode: &OpcodeMode,
    operation: fn(i64) -> bool,
) {
    let first_param = first_param.expect("Expected to have the first parameter");
    let second_param = second_param.expect("Expected to have the second parameter");
    let comparison_result;
    match opcode_mode.first_parameter_mode() {
        ParameterMode::Position => {
            comparison_result = operation(memory_access(memory, first_param, 0))
        }
        ParameterMode::Immediate => comparison_result = operation(first_param),
        ParameterMode::Relative => {
            comparison_result = operation(memory_access(memory, first_param, relative_base_offset))
        }
    }
    if comparison_result {
        match opcode_mode.second_parameter_mode() {
            ParameterMode::Position => {
                *instruction_pointer = memory_access(memory, second_param, 0) as u64
            }
            ParameterMode::Immediate => *instruction_pointer = second_param as u64,
            ParameterMode::Relative => {
                *instruction_pointer =
                    memory_access(memory, second_param, relative_base_offset) as u64
            }
        }
    } else {
        *instruction_pointer += 3;
    }
}

fn comparison(
    first_param: Option<i64>,
    second_param: Option<i64>,
    answer: Option<i64>,
    relative_base_offset: u64,
    memory: &mut HashMap<u64, i64>,
    opcode_mode: &OpcodeMode,
    operation: fn(i64, i64) -> bool,
) {
    let first_param = first_param.expect("Expected to have the first parameter");
    let second_param = second_param.expect("Expected to have the second parameter");
    let answer = answer.expect("Expected to have the answer parameter");
    let comparison_result;
    match opcode_mode.first_parameter_mode() {
        ParameterMode::Position => match opcode_mode.second_parameter_mode() {
            ParameterMode::Position => {
                comparison_result = operation(
                    memory_access(memory, first_param, 0),
                    memory_access(memory, second_param, 0),
                )
            }
            ParameterMode::Immediate => {
                comparison_result = operation(memory_access(memory, first_param, 0), second_param)
            }
            ParameterMode::Relative => {
                comparison_result = operation(
                    memory_access(memory, first_param, 0),
                    memory_access(memory, second_param, relative_base_offset),
                )
            }
        },
        ParameterMode::Immediate => match opcode_mode.second_parameter_mode() {
            ParameterMode::Position => {
                comparison_result = operation(first_param, memory_access(memory, second_param, 0))
            }
            ParameterMode::Immediate => comparison_result = operation(first_param, second_param),
            ParameterMode::Relative => {
                comparison_result = operation(
                    first_param,
                    memory_access(memory, second_param, relative_base_offset),
                )
            }
        },
        ParameterMode::Relative => match opcode_mode.second_parameter_mode() {
            ParameterMode::Position => {
                comparison_result = operation(
                    memory_access(memory, first_param, relative_base_offset),
                    memory_access(memory, second_param, 0),
                )
            }
            ParameterMode::Immediate => {
                comparison_result = operation(
                    memory_access(memory, first_param, relative_base_offset),
                    second_param,
                )
            }
            ParameterMode::Relative => {
                comparison_result = operation(
                    memory_access(memory, first_param, relative_base_offset),
                    memory_access(memory, second_param, relative_base_offset),
                )
            }
        },
    }
    match opcode_mode.answer_parameter_mode() {
        ParameterMode::Position => {
            if comparison_result {
                memory_update(memory, answer, 0, 1)
            } else {
                memory_update(memory, answer, 0, 0)
            }
        }
        ParameterMode::Immediate => panic!("Not supposed to be an immediate mode for writing"),
        ParameterMode::Relative => {
            if comparison_result {
                memory_update(memory, answer, relative_base_offset, 1)
            } else {
                memory_update(memory, answer, relative_base_offset, 0)
            }
        }
    }
}

fn process_opcode_and_param_mode(code: i64) -> OpcodeMode {
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
    let mut parameter_modes: Vec<ParameterMode> = Vec::new();
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
            .parse::<i64>()
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

impl OpcodeMode {
    fn opcode(&self) -> &OpCode {
        &self.opcode
    }

    fn first_parameter_mode(&self) -> ParameterMode {
        self.parameter_modes[0]
    }

    fn second_parameter_mode(&self) -> ParameterMode {
        self.parameter_modes[1]
    }

    fn answer_parameter_mode(&self) -> ParameterMode {
        self.parameter_modes[2]
    }
}

fn run_instructions(
    instruction_pointer: &mut u64,
    memory: &mut HashMap<u64, i64>,
    relative_base_offset: u64,
    positions: &Positions,
    opcode_mode: &OpcodeMode,
    operation: fn(i64, i64) -> i64,
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

    let first_nmb = match opcode_mode.first_parameter_mode() {
        ParameterMode::Position => memory_access(memory, first_param, 0),
        ParameterMode::Immediate => first_param,
        ParameterMode::Relative => memory_access(memory, first_param, relative_base_offset),
    };
    let second_nmb = match opcode_mode.second_parameter_mode() {
        ParameterMode::Position => memory_access(memory, second_param, 0),
        ParameterMode::Immediate => second_param,
        ParameterMode::Relative => memory_access(memory, second_param, relative_base_offset),
    };
    match opcode_mode.answer_parameter_mode() {
        ParameterMode::Position => {
            memory_update(memory, answer, 0, operation(first_nmb, second_nmb))
        }
        ParameterMode::Immediate => panic!("wtf i thought there was no immediate mode for writing"),
        ParameterMode::Relative => memory_update(
            memory,
            answer,
            relative_base_offset,
            operation(first_nmb, second_nmb),
        ),
    }
    *instruction_pointer += INSTRUCTION_LENGTH;
}

struct Positions {
    first_param: Option<i64>,
    second_param: Option<i64>,
    answer: Option<i64>,
}

impl Positions {
    pub(crate) fn first_param(&self) -> Option<i64> {
        self.first_param
    }

    pub(crate) fn second_param(&self) -> Option<i64> {
        self.second_param
    }

    pub(crate) fn answer(&self) -> Option<i64> {
        self.answer
    }
}

fn determine_positions(instruction_pointer: u64, memory: &mut HashMap<u64, i64>) -> Positions {
    let first_param = get_parameter_value(instruction_pointer, 1, memory);
    let second_param = get_parameter_value(instruction_pointer, 2, memory);
    let answer = get_parameter_value(instruction_pointer, 3, memory);
    Positions {
        first_param,
        second_param,
        answer,
    }
}

fn get_parameter_value(
    instruction_pointer: u64,
    position: u64,
    memory: &mut HashMap<u64, i64>,
) -> Option<i64> {
    let ins_length: u64 = memory
        .keys()
        .len()
        .try_into()
        .expect("Expected the conversion to work");
    match instruction_pointer + position {
        x if x < ins_length => Some(memory_access(memory, x as i64, 0)),
        _ => None,
    }
}

pub fn load_program_input(file_name: &str) -> io::Result<Vec<i64>> {
    let program_input = File::open(file_name)?;
    let mut reader = BufReader::new(program_input);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let program = buf
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse: {}", s))
        })
        .collect();
    Ok(program)
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
        assert_eq!(output.parameter_modes, correct_output.parameter_modes);

        let input = 203;
        let correct_output = OpcodeMode {
            opcode: OpCode::Input,
            parameter_modes: vec![
                ParameterMode::Relative,
                ParameterMode::Position,
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
            let mut comp = IntcodeComputer::new(&test_set.0);
            comp.run(&mut vec![10]);
            assert_eq!(comp.output(), &test_set.1)
        }
    }

    #[test]
    fn test_conditional_opcodes() {
        let eq_to = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut comp = IntcodeComputer::new(&eq_to);
        comp.run(&mut vec![8]);
        assert_eq!(comp.output(), &vec![1]);

        comp.load_new_instructions(&eq_to);
        comp.run(&mut vec![4]);
        assert_eq!(comp.output(), &vec![0]);

        let less_than = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        comp.load_new_instructions(&less_than);
        comp.run(&mut vec![3]);
        assert_eq!(comp.output(), &vec![1]);

        comp.load_new_instructions(&less_than);
        comp.run(&mut vec![10]);
        assert_eq!(comp.output(), &vec![0]);

        let eq_to = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        comp.load_new_instructions(&eq_to);
        comp.run(&mut vec![8]);
        assert_eq!(comp.output(), &vec![1]);

        comp.load_new_instructions(&eq_to);
        comp.run(&mut vec![4]);
        assert_eq!(comp.output(), &vec![0]);

        let less_than = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        comp.load_new_instructions(&less_than);
        comp.run(&mut vec![3]);
        assert_eq!(comp.output(), &vec![1]);

        comp.load_new_instructions(&less_than);
        comp.run(&mut vec![10]);
        assert_eq!(comp.output(), &vec![0]);
    }

    #[test]
    fn test_jump_opcodes() {
        let jump = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut comp = IntcodeComputer::new(&jump);
        comp.run(&mut vec![8]);
        assert_eq!(comp.output(), &vec![1]);

        comp.load_new_instructions(&jump);
        comp.run(&mut vec![0]);
        assert_eq!(comp.output(), &vec![0]);

        let jump = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        comp.load_new_instructions(&jump);
        comp.run(&mut vec![8]);
        assert_eq!(comp.output(), &vec![1]);

        comp.load_new_instructions(&jump);
        comp.run(&mut vec![0]);
        assert_eq!(comp.output(), &vec![0]);
    }

    #[test]
    fn test_all_opcodes() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut comp = IntcodeComputer::new(&prog);
        comp.run(&mut vec![4]);
        assert_eq!(comp.output(), &vec![999]);

        comp.load_new_instructions(&prog);
        comp.run(&mut vec![8]);
        assert_eq!(comp.output(), &vec![1000]);

        comp.load_new_instructions(&prog);
        comp.run(&mut vec![11]);
        assert_eq!(comp.output(), &vec![1001]);
    }

    #[test]
    fn test_relative_base_offset() {
        let instructions = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut comp = IntcodeComputer::new(&instructions);
        comp.run(&mut vec![]);
        assert_eq!(comp.output(), &instructions);

        let instructions = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        comp.load_new_instructions(&instructions);
        comp.run(&mut vec![]);
        assert_eq!(comp.output(), &vec![34915192 * 34915192]);

        let instructions = vec![104, 1125899906842624, 99];
        comp.load_new_instructions(&instructions);
        comp.run(&mut vec![]);
        assert_eq!(comp.output(), &vec![1125899906842624])
    }
}
