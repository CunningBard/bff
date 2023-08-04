use std::mem::size_of;
use crate::constants::types::{Bits, Register};

pub const GENERAL_PURPOSE_REGISTER_COUNT: usize = 16;
pub const REGISTER_COUNT: usize = GENERAL_PURPOSE_REGISTER_COUNT + 1; // +1 for stack pointer
pub const BASE_MEMORY_SIZE: usize = 1024 * 1024 / size_of::<Bits>(); // 1 MB
pub const BASE_STACK_SIZE: usize = 1024 * 1024 / size_of::<Bits>(); // 1 MB
pub const STACK_POINTER: Register = 0;
pub const INSTRUCTION_SIZE: u32 = 8;