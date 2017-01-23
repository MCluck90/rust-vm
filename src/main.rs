mod assembler;
mod syntax;
mod tokenizer;

use std::env;
use assembler::Assembler;
use tokenizer::Tokenizer;

fn main() {
    let mut args = env::args();
    if let Some(filename) = args.nth(1) {
        let tokenizer = Tokenizer::new(&filename);
        if let Some(err) = syntax::verify(tokenizer) {
            println!("{}", err);
            return;
        }
        let tokenizer = Tokenizer::new(&filename);
        let (label_table, commands) = Assembler::to_commands(tokenizer);
        for command in commands {
            println!("{:?}", command.to_bytes(&label_table));
        }
    } else {
        panic!("Must provide an input file");
    }
}
