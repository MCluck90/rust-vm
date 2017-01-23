mod assembler;
mod tokenizer;

use std::env;
use assembler::Assembler;
use tokenizer::Tokenizer;

fn main() {
    let mut args = env::args();
    if let Some(filename) = args.nth(1) {
        let tokenizer = Tokenizer::new(&filename);
        let commands = Assembler::to_commands(tokenizer);
        for command in commands {
            println!("{:?}", command);
        }
    } else {
        panic!("Must provide an input file");
    }
}
