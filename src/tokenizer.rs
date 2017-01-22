use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub enum DirectiveType {
    Byte,
    Word
}

#[derive(Clone, Debug)]
pub enum InstructionType {
    Jump,
    JumpRelative,
    NonZeroJump,
    GreaterThanZeroJump,
    LessThanZeroJump,
    EqualZeroJump,

    Move,
    LoadAddress,
    StoreWord,
    LoadWord,
    StoreByte,
    LoadByte,

    Add,
    Subtract,
    Multiply,
    Divide,

    And,
    Or,

    Equal
}

#[derive(Clone, Debug)]
pub enum Register {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
    Reg4,
    Reg5,
    Reg6,
    IO,
    PC,
    SL,
    SP,
    FP,
    SB
}

#[derive(Clone, Debug)]
pub enum Token {
    Character(String),
    Directive(DirectiveType),
    Instruction(InstructionType),
    Integer(i32),
    Register(Register),
    Label(String)
}

pub struct Tokenizer {
    lines: Lines<BufReader<File>>,
    newest_tokens: Vec<Token>
}

impl Tokenizer {
    pub fn new(file_path: &str) -> Tokenizer {
        // Open the file stream
        let file_stream = File::open(file_path);
        if !file_stream.is_ok() {
            panic!("Failed to open: {}", file_path);
        }
        let file_stream = file_stream.ok().unwrap();
        let file = BufReader::new(file_stream);

        Tokenizer {
            lines: file.lines(),
            newest_tokens: vec![]
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.newest_tokens.len() > 0 {
            return Some(self.newest_tokens.remove(0));
        }

        let line = self.lines.next();
        match line {
            Some(line)  => {
                let line = line.unwrap();
                let mut line = line.trim();
                if let Some(start_comment) = line.find("#") {
                    line = &line[..start_comment];
                }
                if line.len() == 0 {
                    return self.next();
                }

                // Parse each token
                for token in line.split_whitespace() {
                    let new_token = match token {
                        ".byte" => Token::Directive(DirectiveType::Byte),
                        ".word" => Token::Directive(DirectiveType::Word),

                        "JMP" => Token::Instruction(InstructionType::Jump),
                        "JMR" => Token::Instruction(InstructionType::JumpRelative),
                        "!0" => Token::Instruction(InstructionType::NonZeroJump),
                        ">0" => Token::Instruction(InstructionType::GreaterThanZeroJump),
                        "<0" => Token::Instruction(InstructionType::LessThanZeroJump),
                        "=0" => Token::Instruction(InstructionType::EqualZeroJump),

                        "MOV" => Token::Instruction(InstructionType::Move),
                        "LDA" => Token::Instruction(InstructionType::LoadAddress),
                        "STW" => Token::Instruction(InstructionType::StoreWord),
                        "LDW" => Token::Instruction(InstructionType::LoadWord),
                        "STB" => Token::Instruction(InstructionType::StoreByte),
                        "LDB" => Token::Instruction(InstructionType::LoadByte),

                        "+" => Token::Instruction(InstructionType::Add),
                        "-" => Token::Instruction(InstructionType::Subtract),
                        "*" => Token::Instruction(InstructionType::Multiply),
                        "/" => Token::Instruction(InstructionType::Divide),

                        "&&" => Token::Instruction(InstructionType::And),
                        "||" => Token::Instruction(InstructionType::Or),

                        "==" => Token::Instruction(InstructionType::Equal),

                        "Reg0" => Token::Register(Register::Reg0),
                        "Reg1" => Token::Register(Register::Reg1),
                        "Reg2" => Token::Register(Register::Reg2),
                        "Reg3" => Token::Register(Register::Reg3),
                        "Reg4" => Token::Register(Register::Reg4),
                        "Reg5" => Token::Register(Register::Reg5),
                        "Reg6" => Token::Register(Register::Reg6),
                        "io" => Token::Register(Register::IO),
                        "pc" => Token::Register(Register::PC),
                        "sl" => Token::Register(Register::SL),
                        "sp" => Token::Register(Register::SP),
                        "fp" => Token::Register(Register::FP),
                        "sb" => Token::Register(Register::SB),

                        _ =>  {
                            let num = token.parse::<i32>();
                            if token.chars().nth(0).unwrap() == '\'' &&
                               token.chars().nth(token.len() - 1).unwrap() == '\'' {
                                Token::Character(token.to_string())
                            } else if num.is_ok() {
                                Token::Integer(num.unwrap())
                            } else {
                                Token::Label(token.to_string())
                            }
                        }
                    };

                    self.newest_tokens.push(new_token);
                }

                // Return the first token
                return Some(self.newest_tokens.remove(0));
            },
            None => {
                return None;
            }
        };
    }
}
