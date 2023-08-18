use crate::constants::instructions::Instruction;

pub struct BFFProgram {
    pub version: (u16, u16, u16),
    pub instructions: Vec<Instruction>,
    pub string_table: Vec<(u32, String)>,
}


impl BFFProgram {
    pub fn new(version: (u16, u16, u16), instructions: Vec<Instruction>, string_table: Vec<(u32, String)>,) -> Self {
        Self {
            version,
            instructions,
            string_table,
        }
    }

    pub fn execute(&mut self) {
        let mut vm = crate::engine::virtual_machine::VirtualMachine::new();
        vm.load_program(self.instructions.clone());
        for (location, string) in &self.string_table {
            vm.store_string(*location, &*string);
        }

        vm.execute_instruction_list();
    }

    pub fn to_bfo_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.version.0.to_le_bytes());
        bytes.extend_from_slice(&self.version.1.to_le_bytes());
        bytes.extend_from_slice(&self.version.2.to_le_bytes());
        bytes.extend_from_slice(&[0, 0]);
        bytes.extend_from_slice(&(self.string_table.len() as u64).to_le_bytes());

        for (location, string) in &self.string_table {
            bytes.extend_from_slice(&(string.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&location.to_le_bytes());

            bytes.extend_from_slice(string.as_bytes());
        }

        for instruction in &self.instructions {
            bytes.extend_from_slice(&instruction.to_bfo_bytes());
        }
        bytes
    }
}