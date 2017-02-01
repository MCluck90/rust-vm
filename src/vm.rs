use std::io;
use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use assembler::{Command, CommandType};
use tokenizer::*;

pub struct VM {
    registers: [i32; 13],
    memory: Vec<u8>
}

impl VM {
    pub fn new(code: Vec<u8>) -> VM {
        // Expand available memory
        const MAX_MEMORY: usize = 10_000_000; // 10MB
        let mut memory = vec![0; MAX_MEMORY];

        // Copy bytecode into memory
        let mut i = code.len();
        while i > 0 {
            i -= 1;
            memory[i] = code[i];
        }
        VM {
            registers: [0; 13],
            memory: memory
        }
    }

    pub fn run(&mut self, start_address: usize) {
        let pc = Register::PC.to_bytecode() as usize;
        self.registers[pc] = start_address as i32;

        loop {
            let address = self.registers[pc] as usize;
            let bytecode = {
                let mut memory = Cursor::new(&mut self.memory[address..]);
                [
                    memory.read_i32::<LittleEndian>().unwrap(),
                    memory.read_i32::<LittleEndian>().unwrap(),
                    memory.read_i32::<LittleEndian>().unwrap(),
                ]
            };

            let command = Command::from_bytecode(&bytecode);
            let running = match command.cmd_type {
                CommandType::Instruction(instruction) =>
                    self.execute(instruction, command.operand1, command.operand2, &bytecode),
                _ => false
            };
            if !running {
                break;
            }

            self.registers[pc] += 12;
        }
    }

    fn execute(&mut self, instruction: InstructionType, op1: Token, op2: Token, bytecode: &[i32; 3]) -> bool {
        match instruction {
            // Add together two registers and store the result in the first
            InstructionType::Add => {
                let destination = bytecode[1] as usize;
                let source = bytecode[2] as usize;
                self.registers[destination] += self.registers[source];
            },

            // Add an immediate value to a register
            InstructionType::AddImmediate => {
                let register = bytecode[1] as usize;
                let value = bytecode[2];
                self.registers[register] += value;
            },

            // Perform a boolean AND on two registers
            InstructionType::And => {
                let reg1 = bytecode[1] as usize;
                let reg2 = bytecode[2] as usize;
                let reg1_value = self.registers[reg1];
                let reg2_value = self.registers[reg2];
                self.registers[reg1] = if reg1_value != 0 && reg2_value != 0 {
                    1
                } else {
                    0
                };
            },

            // Compares the contents of two registers
            // -1 if the first is less than the second
            // 1  if the first is greater than the second
            // 0  if they're equal
            InstructionType::Compare => {
                let reg1 = bytecode[1] as usize;
                let reg2 = bytecode[2] as usize;
                let val1 = self.registers[reg1];
                let val2 = self.registers[reg2];
                self.registers[reg1] = if val1 < val2 {
                    -1
                } else if val1 > val2 {
                    1
                } else {
                    0
                };
            },

            // Converts the ASCII representation of a number to the equivalent integer
            // '5' => 5
            InstructionType::ConvertASCIIToInteger => {
                let mut ascii = self.registers[Register::IO as usize];
                ascii -= '0' as i32;
                self.registers[Register::IO as usize] = if ascii < 0 || ascii > 9 {
                    -1
                } else {
                    ascii
                };
            },

            // Converts an integer value to the equivalent ASCII character
            // 5 => '5'
            InstructionType::ConvertIntegerToASCII => {
                let mut integer = self.registers[Register::IO as usize];
                integer += '0' as i32;
                self.registers[Register::IO as usize] = if integer < 48 || integer > 57 {
                    48
                } else {
                    integer
                };
            },

            // Perform integer division between two registers
            InstructionType::Divide => {
                let destination = bytecode[1] as usize;
                let source = bytecode[2] as usize;
                self.registers[destination] /= self.registers[source];
            },

            // If the contents of a register are greater than 0
            // jump to the specified address
            InstructionType::GreaterThanZeroJump => {
                let register = bytecode[1] as usize;
                let address = bytecode[2];
                // Remove offset that will be automatically applied
                let address = address - 12;
                if self.registers[register] > 0 {
                    self.registers[Register::PC as usize] = address;
                }
            },

            // Take in a character from the user and store it in the IO register
            InstructionType::InputASCII => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let character = input.chars().nth(0).unwrap();
                        self.registers[Register::IO as usize] = character as i32;
                    },
                    Err(err) => println!("error: {}", err)
                }
            },

            // Take in a number from the user and store it in the IO register
            InstructionType::InputInteger => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let num = input.trim().parse::<i32>();
                        match num {
                            Ok(n) => self.registers[Register::IO as usize] = n,
                            Err(err) => println!("error: {}", err)
                        }
                    },
                    Err(err) => println!("error: {}", err)
                }
            },

            // Jump directly to an address
            InstructionType::Jump => {
                let address = bytecode[1];

                // Remove offset that will be automatically applied
                let address = address - 12;
                self.registers[Register::PC as usize] = address;
            },

            // Jumps to an address stored in a register
            InstructionType::JumpRelative => {
                let register = bytecode[1] as usize;
                let address = self.registers[register];

                // Remove offset that will be automatically applied
                let address = address - 12;
                self.registers[Register::PC as usize] = address;
            },

            // If the contents of a register are less than 0
            // jump to the specified address
            InstructionType::LessThanZeroJump => {
                let register = bytecode[1] as usize;
                let address = bytecode[2];
                // Remove offset that will be automatically applied
                let address = address - 12;
                if self.registers[register] < 0 {
                    self.registers[Register::PC as usize] = address;
                }
            },

            // Loads the address of a label into a register
            InstructionType::LoadAddress => {
                let register = bytecode[1] as usize;
                let address = bytecode[2];
                self.registers[register] = address;
            },

            // Let a byte of data from memory and place it into a register
            InstructionType::LoadByte => {
                let register = bytecode[1] as usize;
                let address = bytecode[2] as usize;
                let mut memory = Cursor::new(&mut self.memory[address..]);
                let value = memory.read_u8().unwrap();
                self.registers[register] = value as i32;
            },

            // Print out an ASCII character to stdout
            InstructionType::OutputASCII => {
                print!("{}", (self.registers[Register::IO as usize] as u8) as char);
            },

            // Print out a signed integer to stdout
            InstructionType::OutputInteger => {
                print!("{}", self.registers[Register::IO as usize]);
            },

            // End the program
            InstructionType::End => return false,
            _ => {
                println!("{:?}", instruction);
            }
        };
        true
    }
}
