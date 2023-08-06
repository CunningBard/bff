// for active testing

use bffcore;
use bffcore::constants::instructions::Instruction;
use bffcore::constants::types::{Address, Register};
use bffcore::engine::virtual_machine::VirtualMachine;

fn string_to_instructions(address: u32, register: u8, input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut index = 0;
    while input.len() > index {
        if 4 % (input.len() - index) > 0 {
            let mut byte_array: [u8; 4] = input[index..index + 4].as_bytes().try_into().unwrap();
            let value = u32::from_le_bytes(byte_array);

            instructions.extend(
                vec![
                    Instruction::MoveImmediate(register, value),
                    Instruction::Store(address + index as u32, register, 0)
                ]
            );
            index += 4;
        } else if 2 % (input.len() - index) > 0 {
            let mut byte_array: [u8; 2] = input[index..index + 2].as_bytes().try_into().unwrap();
            let value = u16::from_le_bytes(byte_array);

            instructions.extend(
                vec![
                    Instruction::MoveImmediate(register, value as u32),
                    Instruction::Store(address + index as u32, register, 2)
                ]
            );
            index += 2;
        } else {
            let value = input[index..index + 1].as_bytes()[0] as u32;

            instructions.extend(
                vec![
                    Instruction::MoveImmediate(register, value),
                    Instruction::Store(address + index as u32, register, 3)
                ]
            );
            index += 1;
        }
    }
    instructions
}

fn print_string_as_instructions(address: Address, register: Register, string: String) -> Vec<Instruction>{
    let mut initial_instructions = string_to_instructions(address, register, &string);

    initial_instructions.extend(
        vec![
            Instruction::PushImmediate(string.len() as u32), // length of string
            Instruction::PushImmediate(address), // address of string
            Instruction::PushImmediate(0), // file descriptor
            Instruction::PushImmediate(1), // syscall number
            Instruction::SystemCall
        ]
    );

    initial_instructions
}

fn main(){
    let mut vm = VirtualMachine::new(); // hello world
    let mut instructions =
        print_string_as_instructions(
            0,
            1,
            "Hello World!".to_string()
        );


    println!(" -- Instructions --");
    for inst in &instructions {
        println!("{:?}", inst);
    }

    vm.load_program(instructions);

    println!(" -- VM OutPut --");
    vm.execute_instruction_list();
}