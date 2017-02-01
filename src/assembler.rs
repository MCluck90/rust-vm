use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::collections::HashMap;
use tokenizer::*;

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
    ByteDirective(u8),
    WordDirective(u16),
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
            let mut new_type = CommandType::Unknown;
            match &self.cmd_type {
                &CommandType::Instruction(InstructionType::Add) => {
                    match &self.operand2.token_type {
                        &TokenType::Integer(_) => {
                            new_type = CommandType::Instruction(InstructionType::AddImmediate);
                        },
                        _ => {}
                    };
                },
                _ => {}
            };

            if new_type != CommandType::Unknown {
                self.cmd_type = new_type;
            }
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
                &TokenType::Character(c) => ByteCodeData::ByteDirective(c as u8),
                &TokenType::Integer(val) => ByteCodeData::WordDirective(val as u16),
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

    pub fn from_bytecode(code: &[i32; 3]) -> Command {
        let mut command = Command::new();
        if let Some(directive) = DirectiveType::from_bytecode(code[0]) {
            command.cmd_type = CommandType::Directive(directive);
        } else if let Some(instruction) = InstructionType::from_bytecode(code[0]) {
            match &instruction {
                &InstructionType::Add |
                &InstructionType::And |
                &InstructionType::Divide |
                &InstructionType::Compare |
                &InstructionType::Move |
                &InstructionType::Multiply |
                &InstructionType::Or => {
                    if let Some(register) = Register::from_bytecode(code[1]) {
                        command.operand1 = Token::new(TokenType::Register(register), 0);
                    } else {
                        unreachable!();
                    }

                    if let Some(register) = Register::from_bytecode(code[2]) {
                        command.operand2 = Token::new(TokenType::Register(register), 0);
                    } else {
                        unreachable!();
                    }
                },

                // Takes a register and an offset (written as a label)
                // or a register and an immediate value
                &InstructionType::AddImmediate |
                &InstructionType::GreaterThanZeroJump |
                &InstructionType::LessThanZeroJump |
                &InstructionType::LoadAddress |
                &InstructionType::LoadByte |
                &InstructionType::LoadWord |
                &InstructionType::NonZeroJump |
                &InstructionType::StoreByte => {
                    if let Some(register) = Register::from_bytecode(code[1]) {
                        command.operand1 = Token::new(TokenType::Register(register), 0);
                    } else {
                        unreachable!();
                    }

                    command.operand2 = Token::new(TokenType::Integer(code[2]), 0);
                },

                // Only takes an address (label)
                &InstructionType::Jump => {
                    command.operand1 = Token::new(TokenType::Integer(code[1]), 0);
                },

                // Only takes a register
                &InstructionType::JumpRelative => {
                    if let Some(register) = Register::from_bytecode(code[1]) {
                        command.operand1 = Token::new(TokenType::Register(register), 0);
                    } else {
                        unreachable!();
                    }
                },

                // Don't take any arguments
                &InstructionType::ConvertASCIIToInteger |
                &InstructionType::ConvertIntegerToASCII |
                &InstructionType::End |
                &InstructionType::InputASCII |
                &InstructionType::InputInteger |
                &InstructionType::OutputASCII |
                &InstructionType::OutputInteger => {},

                _ => {
                    println!("Unhandled instruction: {:?}", instruction);
                    panic!("Unhandled case in Command::from_bytecode");
                }
            };
            command.cmd_type = CommandType::Instruction(instruction);
        }
        command
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
            &CompareZeroJump |
            &Move |
            &LoadAddress |
            &StoreWord |
            &LoadWord |
            &StoreByte |
            &LoadByte |
            &Add |
            &AddImmediate |
            &Subtract |
            &Multiply |
            &Divide |
            &And |
            &Or |
            &Compare => !self.operand1.is_none() &&
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

    pub fn to_bytecode(label_table: HashMap<String, i32>, commands: Vec<Command>) -> (usize, Vec<u8>) {
        let mut bytecode = vec![];
        let mut offset = 0;
        let mut start: usize = 0;
        let mut found_start = false;
        for command in commands {
            let code = command.to_bytecode(&label_table);
            match code {
                ByteCodeData::ByteDirective(data) => {
                    bytecode.write_u8(data).unwrap();
                    offset += 1;
                },
                ByteCodeData::WordDirective(data) => {
                    bytecode.write_u16::<LittleEndian>(data).unwrap();
                    offset += 2;
                },
                ByteCodeData::Instruction(data) => {
                    if !found_start {
                        start = offset;
                        found_start = true;
                    }
                    bytecode.write_i32::<LittleEndian>(data[0]).unwrap();
                    bytecode.write_i32::<LittleEndian>(data[1]).unwrap();
                    bytecode.write_i32::<LittleEndian>(data[2]).unwrap();
                    offset += 3;
                }
            };
        }
        (start, bytecode)
    }
}
