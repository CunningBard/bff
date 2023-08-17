use crate::constants::constants::INSTRUCTION_SIZE;
use crate::constants::instructions::Instruction;
use crate::constants::types::{Register};
use crate::engine::program::BFFProgram;

pub struct BFOReader {}


impl BFOReader {
    pub fn read_program(program: Vec<u8>) -> BFFProgram {
        let mut index = 8;
        let version_major = u16::from_le_bytes([program[0], program[1]]);
        let version_minor = u16::from_le_bytes([program[2], program[3]]);
        let version_incremental = u16::from_le_bytes([program[4], program[5]]);
        let version = (version_major, version_minor, version_incremental);

        if version != crate::constants::constants::VERSION {
            eprintln!("Version Mismatch, expected {:?}, got {:?}", crate::constants::constants::VERSION, version);
        }

        let mut instructions = vec![];
        if index + INSTRUCTION_SIZE as usize > program.len() {
            panic!("WELL SHT") // place holder error
        }

        while index < program.len() {
            instructions.push(
                Instruction::from_bfo_bytes(
                    [
                        program[index],
                        program[index + 1],
                        program[index + 2],
                        program[index + 3],
                        program[index + 4],
                        program[index + 5],
                        program[index + 6],
                        program[index + 7],
                    ])
            );
            index += INSTRUCTION_SIZE as usize;
        }

        BFFProgram {
            version,
            instructions,
        }
    }

    pub fn read_file_from_path(path: String) -> BFFProgram {
        let program = match std::fs::read(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Couldn't Read The Program");
                panic!("{}", err);
            }
        };

        Self::read_program(program)
    }
}
