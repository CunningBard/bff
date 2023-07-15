use crate::constants::{BASE_MEMORY_SIZE, BASE_STACK_SIZE, REGISTER_COUNT, STACK_POINTER};
use crate::instructions::Instruction;
use crate::types::{Address, Bits};

#[derive(Debug)]
pub struct VirtualMachine {
    // stack pointer is always register 0, or registers[0]
    pub registers: [Bits; REGISTER_COUNT],
    pub stack: Vec<Bits>,
    pub memory: Vec<Bits>,
    pub call_stack: Vec<Address>,
    pub program_counter: usize,
    pub instruction_list: Vec<Instruction>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; REGISTER_COUNT],
            stack: vec![0; BASE_STACK_SIZE],
            memory: vec![0; BASE_MEMORY_SIZE],
            call_stack: vec![],
            program_counter: 0,
            instruction_list: vec![Instruction::Nop],
        }
    }
    pub fn execute_single_instruction(&mut self){
        let instruction = &self.instruction_list[self.program_counter];
        match *instruction {
            Instruction::Nop => {
                // do nothing
            }
            Instruction::Add(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] + self.registers[rhs as usize];
            }
            Instruction::AddImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] + rhs;
            }
            Instruction::Sub(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] - self.registers[rhs as usize];
            }
            Instruction::SubImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] - rhs;
            }
            Instruction::Mul(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] * self.registers[rhs as usize];
            }
            Instruction::MulImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] * rhs;
            }
            Instruction::DivMod(div_dst, mod_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / self.registers[rhs as usize];
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % self.registers[rhs as usize];
            }
            Instruction::DivModImmediate(div_dst, mod_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / rhs;
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % rhs;
            }
            Instruction::Div(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] / self.registers[rhs as usize];
            }
            Instruction::DivImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] / rhs;
            }
            Instruction::Mod(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] % self.registers[rhs as usize];
            }
            Instruction::ModImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] % rhs;
            }

            Instruction::GreaterThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] > self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::GreaterThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] > rhs { 1 } else { 0 };
            }
            Instruction::LessThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] < self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::LessThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] < rhs { 1 } else { 0 };
            }
            Instruction::GreaterThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] >= self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::GreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] >= rhs { 1 } else { 0 };
            }
            Instruction::LessThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] <= self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::LessThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] <= rhs { 1 } else { 0 };
            }
            Instruction::Equal(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] == self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::EqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] == rhs { 1 } else { 0 };
            }
            Instruction::NotEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] != self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::NotEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] != rhs { 1 } else { 0 };
            }

            Instruction::FloatAdd(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] + self.registers[rhs as usize];
            }
            Instruction::FloatAddImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] + rhs;
            }
            Instruction::FloatSub(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] - self.registers[rhs as usize];
            }
            Instruction::FloatSubImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] - rhs;
            }
            Instruction::FloatMul(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] * self.registers[rhs as usize];
            }
            Instruction::FloatMulImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] * rhs;
            }
            Instruction::FloatDivMod(div_dst, mod_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / self.registers[rhs as usize];
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % self.registers[rhs as usize];
            }
            Instruction::FloatDivModImmediate(div_dst, mod_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / rhs;
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % rhs;
            }
            Instruction::FloatDiv(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] / self.registers[rhs as usize];
            }
            Instruction::FloatDivImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] / rhs;
            }
            Instruction::FloatMod(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] % self.registers[rhs as usize];
            }
            Instruction::FloatModImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] % rhs;
            }

            Instruction::FloatGreaterThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] > self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::FloatGreaterThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] > rhs { 1 } else { 0 };
            }
            Instruction::FloatLessThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] < self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::FloatLessThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] < rhs { 1 } else { 0 };
            }
            Instruction::FloatGreaterThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] >= self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::FloatGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] >= rhs { 1 } else { 0 };
            }
            Instruction::FloatLessThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] <= self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::FloatLessThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] <= rhs { 1 } else { 0 };
            }
            Instruction::FloatEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] == self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::FloatEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] == rhs { 1 } else { 0 };
            }
            Instruction::FloatNotEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] != self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::FloatNotEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] != rhs { 1 } else { 0 };
            }
            Instruction::FloatNegate(dst, src) => {
                self.registers[dst as usize] = -self.registers[src as usize];
            }
            Instruction::FloatNegateImmediate(dst, src) => {
                self.registers[dst as usize] = -src;
            }

            Instruction::SignedAdd(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] + self.registers[rhs as usize];
            }
            Instruction::SignedAddImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] + rhs;
            }
            Instruction::SignedSub(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] - self.registers[rhs as usize];
            }
            Instruction::SignedSubImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] - rhs;
            }
            Instruction::SignedMul(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] * self.registers[rhs as usize];
            }
            Instruction::SignedMulImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] * rhs;
            }
            Instruction::SignedDivMod(div_dst, mod_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / self.registers[rhs as usize];
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % self.registers[rhs as usize];
            }
            Instruction::SignedDivModImmediate(div_dst, mod_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / rhs;
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % rhs;
            }
            Instruction::SignedDiv(div_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / self.registers[rhs as usize];
            }
            Instruction::SignedDivImmediate(div_dst, lhs, rhs) => {
                self.registers[div_dst as usize] = self.registers[lhs as usize] / rhs;
            }
            Instruction::SignedMod(mod_dst, lhs, rhs) => {
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % self.registers[rhs as usize];
            }
            Instruction::SignedModImmediate(mod_dst, lhs, rhs) => {
                self.registers[mod_dst as usize] = self.registers[lhs as usize] % rhs;
            }

            Instruction::SignedGreaterThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] > self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::SignedGreaterThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] > rhs { 1 } else { 0 };
            }
            Instruction::SignedLessThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] < self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::SignedLessThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] < rhs { 1 } else { 0 };
            }
            Instruction::SignedGreaterThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] >= self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::SignedGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] >= rhs { 1 } else { 0 };
            }
            Instruction::SignedLessThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] <= self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::SignedLessThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] <= rhs { 1 } else { 0 };
            }
            Instruction::SignedEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] == self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::SignedEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] == rhs { 1 } else { 0 };
            }
            Instruction::SignedNotEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] != self.registers[rhs as usize] { 1 } else { 0 };
            }
            Instruction::SignedNotEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] != rhs { 1 } else { 0 };
            }
            Instruction::SignedNegate(dst, src) => {
                self.registers[dst as usize] = -self.registers[src as usize];
            }
            Instruction::SignedNegateImmediate(dst, src) => {
                self.registers[dst as usize] = -src;
            }

            Instruction::And(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] & self.registers[rhs as usize];
            }
            Instruction::AndImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] & rhs;
            }
            Instruction::Or(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] | self.registers[rhs as usize];
            }
            Instruction::OrImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] | rhs;
            }
            Instruction::Xor(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] ^ self.registers[rhs as usize];
            }
            Instruction::XorImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] ^ rhs;
            }
            Instruction::Not(dst, src) => {
                self.registers[dst as usize] = !self.registers[src as usize];
            }
            Instruction::NotImmediate(dst, src) => {
                self.registers[dst as usize] = !src;
            }

            Instruction::LeftShift(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] << self.registers[rhs as usize];
            }
            Instruction::LeftShiftImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] << rhs;
            }
            Instruction::RightShift(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] >> self.registers[rhs as usize];
            }
            Instruction::RightShiftImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] >> rhs;
            }

            Instruction::Jump(address) => {
                self.program_counter = (address - 1) as usize;
            }
            Instruction::JumpIfNotZero(src, address) => {
                if self.registers[src as usize] != 0 {
                    self.program_counter = (address - 1) as usize;
                }
            }

            Instruction::Move(dst, src) => {
                self.registers[dst as usize] = self.registers[src as usize];
            }
            Instruction::MoveImmediate(dst, src) => {
                self.registers[dst as usize] = src;
            }

            Instruction::Push(src) => {
                self.stack[self.registers[STACK_POINTER as usize] as usize] = self.registers[src as usize];
                self.registers[STACK_POINTER as usize] += 1;
            }
            Instruction::PushImmediate(src) => {
                self.stack[self.registers[STACK_POINTER as usize] as usize] = src;
                self.registers[STACK_POINTER as usize] += 1;
            }
            Instruction::Pop(dst) => {
                self.registers[STACK_POINTER as usize] -= 1;
                self.registers[dst as usize] = self.stack[self.registers[STACK_POINTER as usize] as usize];
            }

            Instruction::Store(address, src) => {
                self.memory[address as usize] = self.registers[src as usize];
            }
            Instruction::StoreImmediate(address, src) => {
                self.memory[address as usize] = src;
            }
            Instruction::Load(dst, address) => {
                let address = self.registers[address as usize];
                self.registers[dst as usize] = self.memory[address as usize];
            }
            Instruction::LoadImmediate(dst, address) => {
                self.registers[dst as usize] = self.memory[address as usize];
            }

            Instruction::Call(address) => {
                self.call_stack.push(self.program_counter as Address);
                self.program_counter = (address - 1) as usize;
            }
            Instruction::Return => {
                self.program_counter = match self.call_stack.pop() {
                    None => { panic!("Call stack underflow!"); }
                    Some(address) => { address as usize }
                };
            }
        }
    }

    pub fn execute_instruction_list(&mut self) {
        while self.program_counter < self.instruction_list.len() {
            self.execute_single_instruction();
            self.program_counter += 1;
        }
    }
}