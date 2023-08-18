use std::io;
use std::io::Write;
use std::ops::Neg;
use crate::constants::constants::{BASE_MEMORY_SIZE, BASE_STACK_SIZE, REGISTER_COUNT, STACK_POINTER};
use crate::constants::instructions::Instruction;
use crate::constants::types::{Address, Bits, Byte};

#[derive(Debug)]
pub struct VirtualMachine {
    // stack pointer is always register 0, or registers[0]
    pub registers: [Bits; REGISTER_COUNT],
    pub stack: Vec<Bits>,
    pub memory: Vec<Byte>,
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
    pub fn pop_stack(&mut self) -> Bits {
        self.registers[STACK_POINTER as usize] -= 1;
        let value = self.stack[self.registers[STACK_POINTER as usize] as usize];
        value
    }
    pub fn push_stack(&mut self, value: Bits) {
        self.stack[self.registers[STACK_POINTER as usize] as usize] = value;
        self.registers[STACK_POINTER as usize] += 1;
    }
    fn store(&mut self, address: Address, value: Bits, size: u8) {
        match size {
            0 => {
                let value = value.to_le_bytes();
                self.memory[address as usize] = value[0];
                self.memory[address as usize + 1] = value[1];
                self.memory[address as usize + 2] = value[2];
                self.memory[address as usize + 3] = value[3];
            }
            2 => {
                let value = (value as u16).to_le_bytes();
                self.memory[address as usize] = value[0];
                self.memory[address as usize + 1] = value[1];
            }
            3 => {
                self.memory[address as usize] = value as u8;
            }
            _ => {
                panic!("byte size is invalid! for store instruction");
            }
        }
    }
    fn load(&mut self, address: Address, size: u8) -> Bits {
        match size {
            0 => {
                let mut value = [0; 4];
                value[0] = self.memory[address as usize];
                value[1] = self.memory[address as usize + 1];
                value[2] = self.memory[address as usize + 2];
                value[3] = self.memory[address as usize + 3];
                Bits::from_le_bytes(value)
            }
            2 => {
                let mut value = [0; 2];
                value[0] = self.memory[address as usize];
                value[1] = self.memory[address as usize + 1];
                u16::from_le_bytes(value) as Bits
            }
            3 => {
                self.memory[address as usize] as Bits
            }
            _ => {
                panic!("byte size is invalid! for load instruction");
            }
        }
    }
    pub fn store_string(&mut self, address: Address, value: &str) {
        for (index, byte) in value.bytes().into_iter().enumerate() {
            self.memory[address as usize + index] = byte;
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

            Instruction::ShiftLeft(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] << self.registers[rhs as usize];
            }
            Instruction::ShiftLeftImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] << rhs;
            }
            Instruction::ShiftRight(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] >> self.registers[rhs as usize];
            }
            Instruction::ShiftRightImmediate(dst, lhs, rhs) => {
                self.registers[dst as usize] = self.registers[lhs as usize] >> rhs;
            }

            Instruction::Jump(register) => {
                self.program_counter = (self.registers[register as usize] - 1) as usize;
            }
            Instruction::JumpImmediate(address) => {
                self.program_counter = (address - 1) as usize;
            }
            Instruction::JumpNotZero(register, address_register) => {
                if self.registers[register as usize] != 0 {
                    self.program_counter = (self.registers[address_register as usize] - 1) as usize;
                }
            }
            Instruction::JumpNotZeroImmediate(src, address) => {
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
                self.push_stack(self.registers[src as usize])
            }
            Instruction::PushImmediate(src) => {
                self.push_stack(src)
            }
            Instruction::Pop(dst) => {
                self.registers[dst as usize] = self.pop_stack();
            }

            Instruction::Store(address, src, byte_num) => {
                let address = self.registers[address as usize];
                self.store(address, self.registers[src as usize], byte_num)
            }
            Instruction::DirectStore(address, src, byte_num) => {
                self.store(address, self.registers[src as usize], byte_num)
            }
            Instruction::Load(dst, address, byte_num) => {
                self.registers[dst as usize] = self.load(self.registers[address as usize], byte_num);
            }
            Instruction::DirectLoad(dst, address, byte_num) => {
                self.registers[dst as usize] = self.load(address, byte_num);
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
            Instruction::SystemCall => {
                let syscall_num = self.pop_stack();
                match syscall_num {
                    0 => {
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let input = input.trim();
                        let input = input.parse::<u32>().expect("Failed to parse input");
                        self.push_stack(input);
                    }
                    1 => {
                        let file_descriptor = self.pop_stack();
                        let address = self.pop_stack();
                        let length = self.pop_stack();

                        let mut buffer = vec![0u8; length as usize];
                        for i in 0..length {
                            buffer[i as usize] = self.memory[(address + i) as usize];
                        }
                        let string = String::from_utf8(buffer).expect("Failed to parse string");

                        match file_descriptor {
                            0 => {
                                let mut stdout = io::stdout();
                                stdout.write_all(string.as_bytes()).expect("Failed to write to stdout");
                            }
                            1 => {
                                let mut stderr = io::stderr();
                                stderr.write_all(string.as_bytes()).expect("Failed to write to stderr");
                            }
                            _ => {
                                panic!("Invalid file descriptor!");
                            }
                        }
                    }
                    _ => {
                        panic!("Invalid syscall number!");
                    }
                }
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