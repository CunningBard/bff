use crate::constants::instructions::Instruction;

pub struct BFFProgram {
    pub version: (u16, u16, u16),
    pub instructions: Vec<Instruction>
}


impl BFFProgram {
    pub fn new(version: (u16, u16, u16), instructions: Vec<Instruction>) -> Self {
        Self {
            version,
            instructions,
        }
    }

    pub fn execute(&mut self) {
        let mut vm = crate::engine::virtual_machine::VirtualMachine::new();
        vm.load_program(self.instructions.clone());
        vm.execute_instruction_list();
    }
}