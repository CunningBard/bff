use crate::constants::instructions::Instruction;

pub struct BFFProgram {
    pub version: (u16, u16, u16),
    pub instructions: Vec<Instruction>
}