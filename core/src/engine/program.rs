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

    pub fn to_bfo_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.version.0.to_le_bytes());
        bytes.extend_from_slice(&self.version.1.to_le_bytes());
        bytes.extend_from_slice(&self.version.2.to_le_bytes());
        bytes.extend_from_slice(&[0, 0]);
        for instruction in &self.instructions {
            bytes.extend_from_slice(&instruction.to_bfo_bytes());
        }
        bytes
    }
}