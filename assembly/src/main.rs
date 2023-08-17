mod parser;

#[macro_use]
extern crate pest_derive;
extern crate pest;

use std::fs;
use bffcore::constants::constants::VERSION;
use bffcore::engine::program::BFFProgram;
use bffcore::engine::virtual_machine::VirtualMachine;

fn main(){
    let mut parser = parser::BffAsmParser::new();
    let contents = &*fs::read_to_string("./assembly/main.bffasm").expect("Couldnt read file");
    parser.parse(contents).expect("Couldnt parse file");

    let mut bfo_program = BFFProgram::new(VERSION, parser.instructions);

    // write as bfo file
    let compiled = bfo_program.to_bfo_bytes();
    println!("Number of instructions: {:?}", compiled.len());
    assert_eq!(compiled.len() % 8, 0, "Number of instructions is not a multiple of 8");

    fs::write("./assembly/main.bfo", compiled).expect("Couldnt write file");
}