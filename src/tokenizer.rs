use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::iter::Iterator;

#[derive(Clone, Debug, PartialEq)]
pub enum DirectiveType {
    Byte,
    Word
}

#[derive(Clone, Debug, PartialEq)]
pub enum InstructionType {
    End,
    OutputInteger,
    InputInteger,
    OutputASCII,
    InputASCII,
    ConvertASCIIToInteger,
    ConvertIntegerToASCII,

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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Character(String),
    Directive(DirectiveType),
    Instruction(InstructionType),
    Integer(i32),
    Register(Register),
    Label(String),
    None
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TokenType::Character(ref str) => write!(f, "{}", str),
            &TokenType::Directive(ref directive) => match directive {
                &DirectiveType::Byte => write!(f, ".byte"),
                &DirectiveType::Word => write!(f, ".word")
            },
            &TokenType::Instruction(ref instruction) => match instruction {
                &InstructionType::Add => write!(f, "+"),
                &InstructionType::And => write!(f, "&&"),
                &InstructionType::ConvertASCIIToInteger => write!(f, "A2I"),
                &InstructionType::ConvertIntegerToASCII => write!(f, "I2A"),
                &InstructionType::Divide => write!(f, "/"),
                &InstructionType::End => write!(f, "END"),
                &InstructionType::Equal => write!(f, "=="),
                &InstructionType::EqualZeroJump => write!(f, "=0"),
                &InstructionType::GreaterThanZeroJump => write!(f, ">0"),
                &InstructionType::InputASCII => write!(f, "ASCI"),
                &InstructionType::InputInteger => write!(f, "IN"),
                &InstructionType::Jump => write!(f, "JMP"),
                &InstructionType::JumpRelative => write!(f, "JMR"),
                &InstructionType::LessThanZeroJump => write!(f, "<0"),
                &InstructionType::LoadAddress => write!(f, "LDA"),
                &InstructionType::LoadByte => write!(f, "LDB"),
                &InstructionType::LoadWord => write!(f, "LDW"),
                &InstructionType::Move => write!(f, "MOV"),
                &InstructionType::Multiply => write!(f, "*"),
                &InstructionType::NonZeroJump => write!(f, "!0"),
                &InstructionType::Or => write!(f, "||"),
                &InstructionType::OutputASCII => write!(f, "ASCO"),
                &InstructionType::OutputInteger => write!(f, "OUT"),
                &InstructionType::StoreByte => write!(f, "STB"),
                &InstructionType::StoreWord => write!(f, "STW"),
                &InstructionType::Subtract => write!(f, "-")
            },
            &TokenType::Integer(ref val) => write!(f, "{}", val),
            &TokenType::Label(ref label) => write!(f, "{}", label),
            &TokenType::None => write!(f, "None"),
            &TokenType::Register(ref register) => match register {
                &Register::FP => write!(f, "FP"),
                &Register::IO => write!(f, "IO"),
                &Register::PC => write!(f, "PC"),
                &Register::Reg0 => write!(f, "reg_0"),
                &Register::Reg1 => write!(f, "reg_1"),
                &Register::Reg2 => write!(f, "reg_2"),
                &Register::Reg3 => write!(f, "reg_3"),
                &Register::Reg4 => write!(f, "reg_4"),
                &Register::Reg5 => write!(f, "reg_5"),
                &Register::Reg6 => write!(f, "reg_6"),
                &Register::SB => write!(f, "SB"),
                &Register::SL => write!(f, "SL"),
                &Register::SP => write!(f, "SP")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line_number: u32
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token [ {}: {} ]", self.line_number, self.token_type)
    }
}

impl Token {
    pub fn new(t: TokenType, l: u32) -> Token {
        Token {
            token_type: t,
            line_number: l
        }
    }

    pub fn new_none() -> Token {
        Token {
            token_type: TokenType::None,
            line_number: 0
        }
    }

    pub fn is_none(&self) -> bool {
        self.token_type == TokenType::None
    }
}

pub struct Tokenizer {
    lines: Lines<BufReader<File>>,
    newest_tokens: Vec<Token>,
    line_number: u32,
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
            newest_tokens: vec![],
            line_number: 0
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
        self.line_number += 1;
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
                        ".byte" => Token::new(
                            TokenType::Directive(DirectiveType::Byte),
                            self.line_number
                        ),
                        ".word" => Token::new(
                            TokenType::Directive(DirectiveType::Word),
                            self.line_number
                        ),
                        "JMP" => Token::new(
                            TokenType::Instruction(InstructionType::Jump),
                            self.line_number
                        ),
                        "JMR" => Token::new(
                            TokenType::Instruction(InstructionType::JumpRelative),
                            self.line_number
                        ),
                        "!0" => Token::new(
                            TokenType::Instruction(InstructionType::NonZeroJump),
                            self.line_number
                        ),
                        ">0" => Token::new(
                            TokenType::Instruction(InstructionType::GreaterThanZeroJump),
                            self.line_number
                        ),
                        "<0" => Token::new(
                            TokenType::Instruction(InstructionType::LessThanZeroJump),
                            self.line_number
                        ),
                        "=0" => Token::new(
                            TokenType::Instruction(InstructionType::EqualZeroJump),
                            self.line_number
                        ),

                        "MOV" => Token::new(
                            TokenType::Instruction(InstructionType::Move),
                            self.line_number
                        ),
                        "LDA" => Token::new(
                            TokenType::Instruction(InstructionType::LoadAddress),
                            self.line_number
                        ),
                        "STW" => Token::new(
                            TokenType::Instruction(InstructionType::StoreWord),
                            self.line_number
                        ),
                        "LDW" => Token::new(
                            TokenType::Instruction(InstructionType::LoadWord),
                            self.line_number
                        ),
                        "STB" => Token::new(
                            TokenType::Instruction(InstructionType::StoreByte),
                            self.line_number
                        ),
                        "LDB" => Token::new(
                            TokenType::Instruction(InstructionType::LoadByte),
                            self.line_number
                        ),

                        "+" => Token::new(
                            TokenType::Instruction(InstructionType::Add),
                            self.line_number
                        ),
                        "-" => Token::new(
                            TokenType::Instruction(InstructionType::Subtract),
                            self.line_number
                        ),
                        "*" => Token::new(
                            TokenType::Instruction(InstructionType::Multiply),
                            self.line_number
                        ),
                        "/" => Token::new(
                            TokenType::Instruction(InstructionType::Divide),
                            self.line_number
                        ),

                        "&&" => Token::new(
                            TokenType::Instruction(InstructionType::And),
                            self.line_number
                        ),
                        "||" => Token::new(
                            TokenType::Instruction(InstructionType::Or),
                            self.line_number
                        ),

                        "==" => Token::new(
                            TokenType::Instruction(InstructionType::Equal),
                            self.line_number
                        ),

                        "reg_0" => Token::new(
                            TokenType::Register(Register::Reg0),
                            self.line_number
                        ),
                        "reg_1" => Token::new(
                            TokenType::Register(Register::Reg1),
                            self.line_number
                        ),
                        "reg_2" => Token::new(
                            TokenType::Register(Register::Reg2),
                            self.line_number
                        ),
                        "reg_3" => Token::new(
                            TokenType::Register(Register::Reg3),
                            self.line_number
                        ),
                        "reg_4" => Token::new(
                            TokenType::Register(Register::Reg4),
                            self.line_number
                        ),
                        "reg_5" => Token::new(
                            TokenType::Register(Register::Reg5),
                            self.line_number
                        ),
                        "reg_6" => Token::new(
                            TokenType::Register(Register::Reg6),
                            self.line_number
                        ),
                        "io" => Token::new(
                            TokenType::Register(Register::IO),
                            self.line_number
                        ),
                        "pc" => Token::new(
                            TokenType::Register(Register::PC),
                            self.line_number
                        ),
                        "sl" => Token::new(
                            TokenType::Register(Register::SL),
                            self.line_number
                        ),
                        "sp" => Token::new(
                            TokenType::Register(Register::SP),
                            self.line_number
                        ),
                        "fp" => Token::new(
                            TokenType::Register(Register::FP),
                            self.line_number
                        ),
                        "sb" => Token::new(
                            TokenType::Register(Register::SB),
                            self.line_number
                        ),

                        "END" => Token::new(
                            TokenType::Instruction(InstructionType::End),
                            self.line_number
                        ),
                        "OUT" => Token::new(
                            TokenType::Instruction(InstructionType::OutputInteger),
                            self.line_number
                        ),
                        "IN" => Token::new(
                            TokenType::Instruction(InstructionType::InputInteger),
                            self.line_number
                        ),
                        "ASCO" => Token::new(
                            TokenType::Instruction(InstructionType::OutputASCII),
                            self.line_number
                        ),
                        "ASCI" => Token::new(
                            TokenType::Instruction(InstructionType::InputASCII),
                            self.line_number
                        ),
                        "A2I" => Token::new(
                            TokenType::Instruction(InstructionType::ConvertASCIIToInteger),
                            self.line_number
                        ),
                        "I2A" => Token::new(
                            TokenType::Instruction(InstructionType::ConvertIntegerToASCII),
                            self.line_number
                        ),

                        _ =>  {
                            // TODO: Handle characters and escape sequences better
                            let num = token.parse::<i32>();
                            if token.chars().nth(0).unwrap() == '\'' &&
                               token.chars().nth(token.len() - 1).unwrap() == '\'' {
                               Token::new(
                                   TokenType::Character(token.to_string()),
                                   self.line_number
                               )
                            } else if num.is_ok() {
                                Token::new(
                                    TokenType::Integer(num.unwrap()),
                                    self.line_number
                                )
                            } else {
                                Token::new(
                                    TokenType::Label(token.to_string()),
                                    self.line_number
                                )
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
