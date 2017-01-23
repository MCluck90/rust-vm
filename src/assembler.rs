use tokenizer::InstructionType;
use tokenizer::DirectiveType;
use tokenizer::Token;
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

impl Command {
    pub fn new() -> Command {
        Command {
            label: Token::None,
            cmd_type: CommandType::Unknown,
            operand1: Token::None,
            operand2: Token::None
        }
    }

    pub fn add_operand(&mut self, operand: Token) {
        if self.operand1 == Token::None {
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

    fn is_directive_complete(&self) -> bool {
        self.operand1 != Token::None
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
            &JumpRelative => self.operand1 != Token::None,

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
            &Equal => self.operand1 != Token::None &&
                      self.operand2 != Token::None
        }
    }

    fn is_label_complete(&self) -> bool {
        match &self.operand1 {
            &Token::Label(_) => true,
            _ => false
        }
    }
}

pub struct Assembler;

impl Assembler {
    pub fn to_commands(tokens: Tokenizer) -> Vec<Command> {
        // TODO: Add a syntax verification step
        let mut commands: Vec<Command> = Vec::new();
        let mut command = Command::new();
        for token in tokens {
            if command.is_complete() {
                commands.push(command);
                command = Command::new();
            }

            use tokenizer::Token::*;
            match token {
                Instruction(instruction) => {
                    command.cmd_type = CommandType::Instruction(instruction);
                },
                Directive(directive) => {
                    command.cmd_type = CommandType::Directive(directive);
                },
                Label(_) => {
                    if command.cmd_type == CommandType::Unknown {
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
        commands
    }
}
