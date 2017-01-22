mod tokenizer;

use std::env;
use tokenizer::Tokenizer;

fn main() {
    let mut args = env::args();
    if let Some(filename) = args.nth(1) {
        let tokenizer = Tokenizer::new(&filename);
        for token in tokenizer {
            println!("{:?}", token);
        }
    } else {
        panic!("Must provide an input file");
    }
}
