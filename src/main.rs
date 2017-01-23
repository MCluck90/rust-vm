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
        let (start, bytecode) = Assembler::to_bytecode(label_table, commands);
        let mut i: usize = 0;
        println!("DATA");
        while i < start {
            println!("{}", bytecode[i]);
            i += 1;
        }

        println!("\nInstructions");
        while bytecode[i] != 0 {
            println!("{} {} {}", bytecode[i], bytecode[i + 1], bytecode[i + 2]);
            i += 3;
        }
    } else {
        panic!("Must provide an input file");
    }
}
