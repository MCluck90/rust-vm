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
        let MAX_MEMORY = 10_000_000; // 10MB
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
            InstructionType::End => return false,
            _ => {
                println!("{:?}", instruction);
            }
        };
        true
    }
}
