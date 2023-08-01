use crate::constants::instructions::Instruction;

pub struct ProgramReader {
    program: Vec<u8>,
    translated: Vec<Instruction>
}

impl ProgramReader {
    pub fn read_file_from_path(path: String){
        let program = match std::fs::read(path){
            Ok(contents) => { contents }
            Err(err) => {
                eprintln!("Couldn't Read The Program");
                panic!(err);
            }
        };


    }
}