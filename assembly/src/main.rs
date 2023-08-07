mod parser;

#[macro_use]
extern crate pest_derive;
extern crate pest;

use std::fs;

fn main(){
    let mut parser = parser::BffAsmParser::new();
    let contents = &*fs::read_to_string("./assembly/main.bffasm").expect("Couldnt read file");
    parser.parse(contents).expect("Couldnt parse file");
    for instruction in parser.instructions {
        println!("{:?}", instruction);
    }
}