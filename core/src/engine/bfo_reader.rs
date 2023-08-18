use crate::constants::constants::INSTRUCTION_SIZE;
use crate::constants::instructions::Instruction;
use crate::constants::types::{Register};
use crate::engine::program::BFFProgram;


macro_rules! array_from_8_elements_in_array {
    ($array:ident, $index:expr) => {
        [
            $array[$index],
            $array[$index + 1],
            $array[$index + 2],
            $array[$index + 3],
            $array[$index + 4],
            $array[$index + 5],
            $array[$index + 6],
            $array[$index + 7]
        ]
    };
}

macro_rules! array_from_4_elements_in_array {
    ($array:ident, $index:expr) => {
        [
            $array[$index],
            $array[$index + 1],
            $array[$index + 2],
            $array[$index + 3],
        ]
    };
}

pub struct BFOReader {}


impl BFOReader {
    pub fn read_program(program: Vec<u8>) -> BFFProgram {
        let mut index = 16;
        let version_major = u16::from_le_bytes([program[0], program[1]]);
        let version_minor = u16::from_le_bytes([program[2], program[3]]);
        let version_incremental = u16::from_le_bytes([program[4], program[5]]);
        let version = (version_major, version_minor, version_incremental);

        let num_strings = u64::from_le_bytes(
            array_from_8_elements_in_array!(program, 8)
        );

        let mut string_table = vec![];
        for _ in 0..num_strings {
            let string_length = u32::from_le_bytes(
                array_from_4_elements_in_array!(program, index)
            );

            let string_location = u32::from_le_bytes(
                array_from_4_elements_in_array!(program, index + 4)
            );

            index += 8;
            let mut string = String::new();
            for _ in 0..string_length {
                string.push(program[index] as char);
                index += 1;
            }
            string_table.push((string_location, string));
        }


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
                    array_from_8_elements_in_array!(program, index)
                )
            );
            index += INSTRUCTION_SIZE as usize;
        }

        BFFProgram {
            version,
            instructions,
            string_table,
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
