use tokenizer::DirectiveType;
use tokenizer::InstructionType;
use tokenizer::Token;
use tokenizer::TokenType;
use tokenizer::Tokenizer;

fn end_of_file() -> Option<String> {
    Some(format!("Unexpected end of file"))
}

fn error_message(expected: &str, token: &Token) -> Option<String> {
    Some(format!("Line {}: Expected {} but saw \"{}\"", token.line_number, expected, token.token_type))
}

pub fn verify(mut tokens: Tokenizer) -> Option<String> {
    let mut token = tokens.next();
    let mut prev_label = false;
    while token.is_some() {
        let t = token.unwrap();
        match &t.token_type {
            &TokenType::Label(_) => {
                token = tokens.next();
                if !prev_label {
                    prev_label = true;
                    continue;
                } else {
                    return error_message("a label, directive, or instruction", &t);
                }
            },
            &TokenType::Directive(ref directive) => {
                let result = verify_directive(&mut tokens, directive);
                if result.is_some() {
                    return result;
                }
            },
            &TokenType::Instruction(ref instruction) => {
                let result = verify_instruction(&mut tokens, instruction);
                if result.is_some() {
                    return result;
                }
            },
            _ => {
                return error_message("a label, directive, or instruction", &t);
            }
        };
        prev_label = false;
        token = tokens.next();
    }
    None
}

fn verify_directive(tokens: &mut Tokenizer, directive: &DirectiveType) -> Option<String> {
    let next_token = tokens.next();
    if !next_token.is_some() {
        return end_of_file();
    }
    let next_token = next_token.unwrap();
    match directive {
        &DirectiveType::Byte => {
            match &next_token.token_type {
                &TokenType::Character(_) => None,
                _ => error_message("an ASCII character", &next_token)
            }
        },
        &DirectiveType::Word => {
            match &next_token.token_type {
                &TokenType::Integer(_) => None,
                _ => error_message("an integer", &next_token)
            }
        }
    }
}

fn verify_instruction(tokens: &mut Tokenizer, instruction: &InstructionType) -> Option<String> {
    let next_token = tokens.next();
    if !next_token.is_some() {
        return match instruction {
            &InstructionType::OutputASCII |
            &InstructionType::OutputInteger |
            &InstructionType::InputASCII |
            &InstructionType::InputInteger |
            &InstructionType::ConvertASCIIToInteger |
            &InstructionType::ConvertIntegerToASCII |
            &InstructionType::End => return None,
            _ => end_of_file()
        };
    }
    let next_token = next_token.unwrap();
    match instruction {
        &InstructionType::Jump => {
            match &next_token.token_type {
                &TokenType::Label(_) => None,
                _ => error_message("a label", &next_token)
            }
        },
        &InstructionType::JumpRelative => {
            match &next_token.token_type {
                &TokenType::Register(_) => None,
                _ => error_message("a register", &next_token)
            }
        },
        &InstructionType::NonZeroJump |
        &InstructionType::GreaterThanZeroJump |
        &InstructionType::LessThanZeroJump |
        &InstructionType::EqualZeroJump |
        &InstructionType::LoadAddress |
        &InstructionType::StoreWord |
        &InstructionType::LoadWord |
        &InstructionType::StoreByte |
        &InstructionType::LoadByte => {
            let second_op = tokens.next();
            if !second_op.is_some() {
                return end_of_file();
            }
            let second_op = second_op.unwrap();

            match &next_token.token_type {
                &TokenType::Register(_) => match &second_op.token_type {
                    &TokenType::Label(_) => None,
                    _ => error_message("a label", &second_op)
                },
                _ => error_message("a register", &next_token)
            }
        },

        &InstructionType::Move |
        &InstructionType::Subtract |
        &InstructionType::Multiply |
        &InstructionType::Divide |
        &InstructionType::And |
        &InstructionType::Or |
        &InstructionType::Equal => {
            let second_op = tokens.next();
            if !second_op.is_some() {
                return end_of_file();
            }
            let second_op = second_op.unwrap();

            match &next_token.token_type {
                &TokenType::Register(_) => match &second_op.token_type {
                    &TokenType::Register(_) => None,
                    _ => error_message("a register", &second_op)
                },
                _ => error_message("a register", &next_token)
            }
        },

        &InstructionType::Add |
        &InstructionType::AddImmediate => {
            let second_op = tokens.next();
            if !second_op.is_some() {
                return end_of_file();
            }
            let second_op = second_op.unwrap();

            match &next_token.token_type {
                &TokenType::Register(_) => match &second_op.token_type {
                    &TokenType::Register(_) |
                    &TokenType::Integer(_) => None,
                    _ => error_message("a register or an integer", &second_op)
                },
                _ => error_message("a register", &next_token)
            }
        },

        &InstructionType::OutputASCII |
        &InstructionType::OutputInteger |
        &InstructionType::InputASCII |
        &InstructionType::InputInteger |
        &InstructionType::ConvertASCIIToInteger |
        &InstructionType::ConvertIntegerToASCII |
        &InstructionType::End => None
    }
}
