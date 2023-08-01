use std::ops::Neg;
use crate::constants::constants::{BASE_MEMORY_SIZE, BASE_STACK_SIZE, REGISTER_COUNT, STACK_POINTER};
use crate::constants::instructions::Instruction;
use crate::constants::types::{Address, Bits};

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
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) + f32::from_bits(self.registers[rhs as usize])).to_bits();
            }
            Instruction::FloatAddImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) + f32::from_bits(rhs)).to_bits();
            }
            Instruction::FloatSub(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) - f32::from_bits(self.registers[rhs as usize])).to_bits();
            }
            Instruction::FloatSubImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) - f32::from_bits(rhs)).to_bits();
            }
            Instruction::FloatMul(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) * f32::from_bits(self.registers[rhs as usize])).to_bits();
            }
            Instruction::FloatMulImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) * f32::from_bits(rhs)).to_bits();
            }
            Instruction::FloatDiv(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) / f32::from_bits(self.registers[rhs as usize])).to_bits();
            }
            Instruction::FloatDivImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) / f32::from_bits(rhs)).to_bits();
            }
            Instruction::FloatMod(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) % f32::from_bits(self.registers[rhs as usize])).to_bits();
            }
            Instruction::FloatModImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (f32::from_bits(self.registers[lhs as usize]) % f32::from_bits(rhs)).to_bits();
            }
            Instruction::FloatDivMod(dst_div, dst_mod, lhs, rhs) => {
                self.registers[dst_div as usize] = (f32::from_bits(self.registers[lhs as usize]) / f32::from_bits(self.registers[rhs as usize])).to_bits();
                self.registers[dst_mod as usize] = (f32::from_bits(self.registers[lhs as usize]) % f32::from_bits(self.registers[rhs as usize])).to_bits();
            }
            Instruction::FloatDivModImmediate(dst_div, dst_mod, lhs, rhs) => {
                self.registers[dst_div as usize] = (f32::from_bits(self.registers[lhs as usize]) / f32::from_bits(rhs)).to_bits();
                self.registers[dst_mod as usize] = (f32::from_bits(self.registers[lhs as usize]) % f32::from_bits(rhs)).to_bits();
            }

            Instruction::FloatGreaterThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) > f32::from_bits(self.registers[rhs as usize]) { 1 } else { 0 };
            }
            Instruction::FloatGreaterThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) > f32::from_bits(rhs) { 1 } else { 0 };
            }
            Instruction::FloatLessThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) < f32::from_bits(self.registers[rhs as usize]) { 1 } else { 0 };
            }
            Instruction::FloatLessThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) < f32::from_bits(rhs) { 1 } else { 0 };
            }
            Instruction::FloatGreaterThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) >= f32::from_bits(self.registers[rhs as usize]) { 1 } else { 0 };
            }
            Instruction::FloatGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) >= f32::from_bits(rhs) { 1 } else { 0 };
            }
            Instruction::FloatLessThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) <= f32::from_bits(self.registers[rhs as usize]) { 1 } else { 0 };
            }
            Instruction::FloatLessThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) <= f32::from_bits(rhs) { 1 } else { 0 };
            }
            Instruction::FloatEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) == f32::from_bits(self.registers[rhs as usize]) { 1 } else { 0 };
            }
            Instruction::FloatEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) == f32::from_bits(rhs) { 1 } else { 0 };
            }
            Instruction::FloatNotEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) != f32::from_bits(self.registers[rhs as usize]) { 1 } else { 0 };
            }
            Instruction::FloatNotEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if f32::from_bits(self.registers[lhs as usize]) != f32::from_bits(rhs) { 1 } else { 0 };
            }
            Instruction::FloatNegate(dst, src) => {
                self.registers[dst as usize] = f32::from_bits(self.registers[src as usize]).neg().to_bits();
            }
            Instruction::FloatNegateImmediate(dst, src) => {
                self.registers[dst as usize] = f32::from_bits(src).neg().to_bits();
            }

            Instruction::SignedAdd(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 + self.registers[rhs as usize] as i32) as u32;
            }
            Instruction::SignedAddImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 + rhs as i32) as u32;
            }
            Instruction::SignedSub(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 - self.registers[rhs as usize] as i32) as u32;
            }
            Instruction::SignedSubImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 - rhs as i32) as u32;
            }
            Instruction::SignedMul(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 * self.registers[rhs as usize] as i32) as u32;
            }
            Instruction::SignedMulImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 * rhs as i32) as u32;
            }
            Instruction::SignedDiv(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 / self.registers[rhs as usize] as i32) as u32;
            }
            Instruction::SignedDivImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 / rhs as i32) as u32;
            }
            Instruction::SignedMod(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 % self.registers[rhs as usize] as i32) as u32;
            }
            Instruction::SignedModImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = (self.registers[lhs as usize] as i32 % rhs as i32) as u32;
            }
            Instruction::SignedDivMod(dst_div, dst_mod, lhs, rhs) => {
                let lhs = self.registers[lhs as usize] as i32;
                let rhs = self.registers[rhs as usize] as i32;
                self.registers[dst_div as usize] = (lhs / rhs) as u32;
                self.registers[dst_mod as usize] = (lhs % rhs) as u32;
            }
            Instruction::SignedDivModImmediate(dst_div, dst_mod, lhs, rhs) => {
                let lhs = self.registers[lhs as usize] as i32;
                let rhs = rhs as i32;
                self.registers[dst_div as usize] = (lhs / rhs) as u32;
                self.registers[dst_mod as usize] = (lhs % rhs) as u32;
            }

            Instruction::SignedGreaterThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 > self.registers[rhs as usize] as i32 { 1 } else { 0 };
            }
            Instruction::SignedGreaterThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 > rhs as i32 { 1 } else { 0 };
            }
            Instruction::SignedGreaterThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 >= self.registers[rhs as usize] as i32 { 1 } else { 0 };
            }
            Instruction::SignedGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 >= rhs as i32 { 1 } else { 0 };
            }
            Instruction::SignedLessThan(dst, lhs, rhs) => {
                self.registers[dst as usize] = if (self.registers[lhs as usize] as i32) < (self.registers[rhs as usize] as i32) { 1 } else { 0 };
            }
            Instruction::SignedLessThanImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if (self.registers[lhs as usize] as i32) < (rhs as i32) { 1 } else { 0 };
            }
            Instruction::SignedLessThanOrEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 <= self.registers[rhs as usize] as i32 { 1 } else { 0 };
            }
            Instruction::SignedLessThanOrEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 <= rhs as i32 { 1 } else { 0 };
            }
            Instruction::SignedEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 == self.registers[rhs as usize] as i32 { 1 } else { 0 };
            }
            Instruction::SignedEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 == rhs as i32 { 1 } else { 0 };
            }
            Instruction::SignedNotEqual(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 != self.registers[rhs as usize] as i32 { 1 } else { 0 };
            }
            Instruction::SignedNotEqualImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = if self.registers[lhs as usize] as i32 != rhs as i32 { 1 } else { 0 };
            }
            Instruction::SignedNegate(dst, src) => {
                self.registers[dst as usize] = -(self.registers[src as usize] as i32) as u32;
            }
            Instruction::SignedNegateImmediate(dst, src) => {
                self.registers[dst as usize] = -(src as i32) as u32;
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

            Instruction::Jump(register) => {
                self.program_counter = (self.registers[register as usize] - 1) as usize;
            }
            Instruction::JumpImmediate(address) => {
                self.program_counter = (address - 1) as usize;
            }
            Instruction::JumpIfNotZero(register, address_register) => {
                if self.registers[register as usize] != 0 {
                    self.program_counter = (self.registers[address_register as usize] - 1) as usize;
                }
            }
            Instruction::JumpIfNotZeroImmediate(src, address) => {
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

    pub fn load_program(&mut self, instruction_list: Vec<Instruction>) {
        self.instruction_list = vec![Instruction::Nop];
        self.instruction_list.extend(instruction_list);
    }
}