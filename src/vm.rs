use assembler::Command;
use assembler::CommandType;
use tokenizer::ByteCode;
use tokenizer::InstructionType;
use tokenizer::Register;
use tokenizer::Token;
use tokenizer::TokenType;

pub struct VM {
    registers: [i32; 13],
    memory: Vec<i32>
}

impl VM {
    pub fn new(code: Vec<i32>) -> VM {
        VM {
            registers: [0; 13],
            memory: code
        }
    }

    pub fn run(&mut self, start_address: usize) {
        let PC = Register::PC.to_bytecode() as usize;
        self.registers[PC] = start_address as i32;

        loop {
            let address = self.registers[PC] as usize;
            let bytecode = [self.memory[address], self.memory[address + 1], self.memory[address + 2]];
            let command = Command::from_bytecode(&bytecode);
            let running = match command.cmd_type {
                CommandType::Instruction(instruction) =>
                    self.execute(instruction, command.operand1, command.operand2),
                _ => false
            };
            if !running {
                break;
            }
        }
    }

    fn execute(&mut self, instruction: InstructionType, op1: Token, op2: Token) -> bool {
        match instruction {
            InstructionType::End => return false,
            _ => {}
        };
        true
    }
}
