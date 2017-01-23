use std::collections::HashMap;

use tokenizer::ByteCode;
use tokenizer::InstructionType;
use tokenizer::DirectiveType;
use tokenizer::Token;
use tokenizer::TokenType;
use tokenizer::Tokenizer;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Directive(DirectiveType),
    Instruction(InstructionType),
    Unknown
}

#[derive(Debug)]
pub struct Command {
    pub label: Token,
    pub cmd_type: CommandType,
    pub operand1: Token,
    pub operand2: Token
}

pub enum ByteCodeData {
    Directive(i32),
    Instruction([i32; 3])
}

impl Command {
    pub fn new() -> Command {
        Command {
            label: Token::new_none(),
            cmd_type: CommandType::Unknown,
            operand1: Token::new_none(),
            operand2: Token::new_none()
        }
    }

    pub fn add_operand(&mut self, operand: Token) {
        if self.operand1.is_none() {
            self.operand1 = operand;
        } else {
            self.operand2 = operand;
        }
    }

    pub fn is_complete(&self) -> bool {
        match &self.cmd_type {
            &CommandType::Directive(_) => self.is_directive_complete(),
            &CommandType::Instruction(ref instruction) =>
                self.is_instruction_complete(&instruction),
            _ => false
        }
    }

    pub fn to_bytecode(&self, label_table: &HashMap<String, i32>) -> ByteCodeData {
        match &self.cmd_type {
            &CommandType::Directive(_) => match &self.operand1.token_type {
                &TokenType::Character(c) => ByteCodeData::Directive((c as u8) as i32),
                &TokenType::Integer(val) => ByteCodeData::Directive(val),
                _ => unreachable!()
            },
            &CommandType::Instruction(ref instruction) => {
                let mut result = [0, 0, 0];
                result[0] = instruction.to_bytecode();
                result[1] = match &self.operand1.token_type {
                    &TokenType::Character(c) => (c as u8) as i32,
                    &TokenType::Integer(val) => val,
                    &TokenType::Register(ref reg) => reg.to_bytecode(),
                    &TokenType::Label(ref label) => match label_table.get(label) {
                        Some(offset) => offset.clone(),

                        // TODO: Better error handling
                        None => panic!("Unknown label")
                    },
                    _ => 0,
                };
                result[2] = match &self.operand2.token_type {
                    &TokenType::Character(c) => (c as u8) as i32,
                    &TokenType::Integer(val) => val,
                    &TokenType::Register(ref reg) => reg.to_bytecode(),
                    &TokenType::Label(ref label) => match label_table.get(label) {
                        Some(offset) => offset.clone(),

                        // TODO: Better error handling
                        None => panic!("Unknown label")
                    },
                    _ => 0,
                };
                ByteCodeData::Instruction(result)
            },
            &CommandType::Unknown => unreachable!()
        }
    }

    fn is_directive_complete(&self) -> bool {
        !self.operand1.is_none()
    }

    fn is_instruction_complete(&self, instruction: &InstructionType) -> bool {
        use tokenizer::InstructionType::*;
        match instruction {
            &End |
            &OutputInteger |
            &InputInteger |
            &OutputASCII |
            &InputASCII |
            &ConvertASCIIToInteger |
            &ConvertIntegerToASCII => true,

            &Jump |
            &JumpRelative => !self.operand1.is_none(),

            &NonZeroJump |
            &GreaterThanZeroJump |
            &LessThanZeroJump |
            &EqualZeroJump |
            &Move |
            &LoadAddress |
            &StoreWord |
            &LoadWord |
            &StoreByte |
            &LoadByte |
            &Add |
            &Subtract |
            &Multiply |
            &Divide |
            &And |
            &Or |
            &Equal => !self.operand1.is_none() &&
                      !self.operand2.is_none()
        }
    }
}

pub struct Assembler;

impl Assembler {
    pub fn to_commands(tokens: Tokenizer) -> (HashMap<String, i32>, Vec<Command>) {
        let mut label_addresses = HashMap::new();
        let mut offset = 0;
        let mut commands: Vec<Command> = Vec::new();
        let mut command = Command::new();
        for token in tokens {
            if command.is_complete() {
                offset += 12;
                commands.push(command);
                command = Command::new();
            }

            use tokenizer::TokenType::*;
            match token.token_type {
                Instruction(instruction) => {
                    command.cmd_type = CommandType::Instruction(instruction);
                },
                Directive(directive) => {
                    command.cmd_type = CommandType::Directive(directive);
                },
                Label(_) => {
                    if command.cmd_type == CommandType::Unknown {
                        if let Label(ref label) = token.token_type {
                            label_addresses.insert(label.to_string(), offset);
                        }
                        command.label = token;
                    } else {
                        command.add_operand(token);
                    }
                },
                _ => {
                    command.add_operand(token);
                }
            };
        }
        if command.is_complete() {
            commands.push(command);
        }
        (label_addresses, commands)
    }

    pub fn to_bytecode(label_table: HashMap<String, i32>, commands: Vec<Command>) -> (usize, Vec<i32>) {
        let mut bytecode = vec![0; 10_000];
        let mut offset = 0;
        let mut start: usize = 0;
        let mut found_start = false;
        for command in commands {
            let code = command.to_bytecode(&label_table);
            match code {
                ByteCodeData::Directive(data) => {
                    bytecode[offset] = data;
                    offset += 1;
                },
                ByteCodeData::Instruction(data) => {
                    if !found_start {
                        start = offset;
                        found_start = true;
                    }
                    bytecode[offset]     = data[0];
                    bytecode[offset + 1] = data[1];
                    bytecode[offset + 2] = data[2];
                    offset += 1;
                }
            };
        }
        (start, bytecode)
    }
}
