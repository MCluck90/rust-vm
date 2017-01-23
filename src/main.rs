mod assembler;
mod syntax;
mod tokenizer;
mod vm;

use std::env;
use assembler::Assembler;
use tokenizer::Tokenizer;
use vm::VM;

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
        let mut vm = VM::new(bytecode);
        vm.run(start);
    } else {
        panic!("Must provide an input file");
    }
}
