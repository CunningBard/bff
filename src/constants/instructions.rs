use crate::constants::constants::INSTRUCTION_SIZE;
use crate::constants::types::{Address, Bits, Register};

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Nop,

    Add(Register, Register, Register),
    AddImmediate(Register, Register, Bits),
    Sub(Register, Register, Register),
    SubImmediate(Register, Register, Bits),
    Mul(Register, Register, Register),
    MulImmediate(Register, Register, Bits),
    Div(Register, Register, Register),
    DivImmediate(Register, Register, Bits),
    Mod(Register, Register, Register),
    ModImmediate(Register, Register, Bits),
    DivMod(Register, Register, Register, Register),
    DivModImmediate(Register, Register, Register, Bits),

    GreaterThan(Register, Register, Register),
    GreaterThanImmediate(Register, Register, Bits),
    LessThan(Register, Register, Register),
    LessThanImmediate(Register, Register, Bits),
    GreaterThanOrEqual(Register, Register, Register),
    GreaterThanOrEqualImmediate(Register, Register, Bits),
    LessThanOrEqual(Register, Register, Register),
    LessThanOrEqualImmediate(Register, Register, Bits),
    Equal(Register, Register, Register),
    EqualImmediate(Register, Register, Bits),
    NotEqual(Register, Register, Register),
    NotEqualImmediate(Register, Register, Bits),

    FloatAdd(Register, Register, Register),
    FloatAddImmediate(Register, Register, Bits),
    FloatSub(Register, Register, Register),
    FloatSubImmediate(Register, Register, Bits),
    FloatMul(Register, Register, Register),
    FloatMulImmediate(Register, Register, Bits),
    FloatDiv(Register, Register, Register),
    FloatDivImmediate(Register, Register, Bits),
    FloatMod(Register, Register, Register),
    FloatModImmediate(Register, Register, Bits),
    FloatDivMod(Register, Register, Register, Register),
    FloatDivModImmediate(Register, Register, Register, Bits),

    FloatGreaterThan(Register, Register, Register),
    FloatGreaterThanImmediate(Register, Register, Bits),
    FloatLessThan(Register, Register, Register),
    FloatLessThanImmediate(Register, Register, Bits),
    FloatGreaterThanOrEqual(Register, Register, Register),
    FloatGreaterThanOrEqualImmediate(Register, Register, Bits),
    FloatLessThanOrEqual(Register, Register, Register),
    FloatLessThanOrEqualImmediate(Register, Register, Bits),
    FloatEqual(Register, Register, Register),
    FloatEqualImmediate(Register, Register, Bits),
    FloatNotEqual(Register, Register, Register),
    FloatNotEqualImmediate(Register, Register, Bits),
    FloatNegate(Register, Register),
    FloatNegateImmediate(Register, Bits),

    SignedAdd(Register, Register, Register),
    SignedAddImmediate(Register, Register, Bits),
    SignedSub(Register, Register, Register),
    SignedSubImmediate(Register, Register, Bits),
    SignedMul(Register, Register, Register),
    SignedMulImmediate(Register, Register, Bits),
    SignedDiv(Register, Register, Register),
    SignedDivImmediate(Register, Register, Bits),
    SignedMod(Register, Register, Register),
    SignedModImmediate(Register, Register, Bits),
    SignedDivMod(Register, Register, Register, Register),
    SignedDivModImmediate(Register, Register, Register, Bits),

    SignedGreaterThan(Register, Register, Register),
    SignedGreaterThanImmediate(Register, Register, Bits),
    SignedLessThan(Register, Register, Register),
    SignedLessThanImmediate(Register, Register, Bits),
    SignedGreaterThanOrEqual(Register, Register, Register),
    SignedGreaterThanOrEqualImmediate(Register, Register, Bits),
    SignedLessThanOrEqual(Register, Register, Register),
    SignedLessThanOrEqualImmediate(Register, Register, Bits),
    SignedEqual(Register, Register, Register),
    SignedEqualImmediate(Register, Register, Bits),
    SignedNotEqual(Register, Register, Register),
    SignedNotEqualImmediate(Register, Register, Bits),
    SignedNegate(Register, Register),
    SignedNegateImmediate(Register, Bits),

    Not(Register, Register),
    NotImmediate(Register, Bits),
    And(Register, Register, Register),
    AndImmediate(Register, Register, Bits),
    Or(Register, Register, Register),
    OrImmediate(Register, Register, Bits),
    Xor(Register, Register, Register),
    XorImmediate(Register, Register, Bits),
    ShiftLeft(Register, Register, Register),
    ShiftLeftImmediate(Register, Register, Bits),
    ShiftRight(Register, Register, Register),
    ShiftRightImmediate(Register, Register, Bits),

    Jump(Register),
    JumpImmediate(Address),
    JumpNotZero(Register, Register),
    JumpNotZeroImmediate(Register, Address),

    Move(Register, Register),
    MoveImmediate(Register, Bits),

    Push(Register),
    PushImmediate(Bits),
    Pop(Register),

    Store(Address, Register),
    Load(Register, Register),

    Call(Address),
    Return,
}

fn get_ar(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Address, Register) {
    let address = u32::from_le_bytes([
        program[index],
        program[index + 1],
        program[index + 2],
        program[index + 3],
    ]);
    let reg = program[index + 4];

    (address, reg)
}

fn get_ra(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Address) {
    let reg = program[index];
    let address = u32::from_le_bytes([
        program[index + 1],
        program[index + 2],
        program[index + 3],
        program[index + 4],
    ]);

    (reg, address)
}

fn get_r(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> Register {
    program[index]
}

fn get_b(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> Bits {
    u32::from_le_bytes([
        program[index],
        program[index + 1],
        program[index + 2],
        program[index + 3],
    ])
}

fn get_rr(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Register) {
    let reg1 = program[index];
    let reg2 = program[index + 1];

    (reg1, reg2)
}

fn get_rb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Bits) {
    let reg = program[index];
    let bits = u32::from_le_bytes([
        program[index + 1],
        program[index + 2],
        program[index + 3],
        program[index + 4],
    ]);

    (reg, bits)
}

fn get_rrb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Register, Bits) {
    let reg1 = program[index];
    let reg2 = program[index + 1];
    let bits = u32::from_le_bytes([
        program[index + 2],
        program[index + 3],
        program[index + 4],
        program[index + 5],
    ]);

    (reg1, reg2, bits)
}

fn get_rrr(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Register, Register) {
    let reg1 = program[index];
    let reg2 = program[index + 1];
    let reg3 = program[index + 2];

    (reg1, reg2, reg3)
}

fn get_rrrr(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Register, Register, Register) {
    let reg1 = program[index];
    let reg2 = program[index + 1];
    let reg3 = program[index + 2];
    let reg4 = program[index + 3];

    (reg1, reg2, reg3, reg4)
}

fn get_rrrb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (Register, Register, Register, Bits) {
    let reg1 = program[index];
    let reg2 = program[index + 1];
    let reg3 = program[index + 2];
    let bits = u32::from_le_bytes([
        program[index + 3],
        program[index + 4],
        program[index + 5],
        program[index + 6],
    ]);

    (reg1, reg2, reg3, bits)
}

impl Instruction {
    pub fn to_bfo_bytes(&self) -> [u8; INSTRUCTION_SIZE as usize]{
        match self {
            Instruction::Nop => {
                [0; INSTRUCTION_SIZE as usize]
            }

            Instruction::Add(dst, lhs, rhs) => {
                [1, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::AddImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [2, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Sub(dst, lhs, rhs) => {
                [3, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SubImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [4, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Mul(dst, lhs, rhs) => {
                [5, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::MulImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [6, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Div(dst, lhs, rhs) => {
                [7, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::DivImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [8, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Mod(dst, lhs, rhs) => {
                [9, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::ModImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [10, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::DivMod(dst1, dst2, lhs, rhs) => {
                [11, *dst1, *dst2, *lhs, *rhs, 0, 0, 0]
            }
            Instruction::DivModImmediate(dst1, dst2, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [12, *dst1, *dst2, *lhs, rhs[0], rhs[1], rhs[2], rhs[3]]
            }

            Instruction::GreaterThan(dst, lhs, rhs) => {
                [13, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::GreaterThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [14, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::LessThan(dst, lhs, rhs) => {
                [15, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::LessThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [16, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::GreaterThanOrEqual(dst, lhs, rhs) => {
                [17, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::GreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [18, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::LessThanOrEqual(dst, lhs, rhs) => {
                [19, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::LessThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [20, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Equal(dst, lhs, rhs) => {
                [21, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::EqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [22, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::NotEqual(dst, lhs, rhs) => {
                [23, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::NotEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [24, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }

            Instruction::FloatAdd(dst, lhs, rhs) => {
                [25, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatAddImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [26, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatSub(dst, lhs, rhs) => {
                [27, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatSubImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [28, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatMul(dst, lhs, rhs) => {
                [29, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatMulImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [30, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatDiv(dst, lhs, rhs) => {
                [31, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatDivImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [32, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatMod(dst, lhs, rhs) => {
                [33, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatModImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [34, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatDivMod(dst1, dst2, lhs, rhs) => {
                [35, *dst1, *dst2, *lhs, *rhs, 0, 0, 0]
            }
            Instruction::FloatDivModImmediate(dst1, dst2, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [36, *dst1, *dst2, *lhs, rhs[0], rhs[1], rhs[2], rhs[3]]
            }

            Instruction::FloatGreaterThan(dst, lhs, rhs) => {
                [37, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatGreaterThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [38, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatLessThan(dst, lhs, rhs) => {
                [39, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatLessThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [40, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatGreaterThanOrEqual(dst, lhs, rhs) => {
                [41, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [42, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatLessThanOrEqual(dst, lhs, rhs) => {
                [43, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatLessThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [44, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatEqual(dst, lhs, rhs) => {
                [45, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [46, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatNotEqual(dst, lhs, rhs) => {
                [47, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::FloatNotEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [48, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::FloatNegate(dst, src) => {
                [49, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::FloatNegateImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [50, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }

            Instruction::SignedAdd(dst, lhs, rhs) => {
                [51, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedAddImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [52, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedSub(dst, lhs, rhs) => {
                [53, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedSubImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [54, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedMul(dst, lhs, rhs) => {
                [55, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedMulImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [56, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedDiv(dst, lhs, rhs) => {
                [57, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedDivImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [58, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedMod(dst, lhs, rhs) => {
                [59, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedModImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [60, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedDivMod(dst, dst2, lhs, rhs) => {
                [61, *dst, *dst2, *lhs, *rhs, 0, 0, 0]
            }
            Instruction::SignedDivModImmediate(dst, dst2, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [62, *dst, *dst2, *lhs, 0, rhs[0], rhs[1], rhs[2]]
            }

            Instruction::SignedGreaterThan(dst, lhs, rhs) => {
                [63, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedGreaterThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [64, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedLessThan(dst, lhs, rhs) => {
                [65, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedLessThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [66, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedGreaterThanOrEqual(dst, lhs, rhs) => {
                [67, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [68, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedLessThanOrEqual(dst, lhs, rhs) => {
                [69, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedLessThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [70, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedEqual(dst, lhs, rhs) => {
                [71, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [72, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedNotEqual(dst, lhs, rhs) => {
                [73, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedNotEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [74, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedNegate(dst, src) => {
                [75, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::SignedNegateImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [76, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }

            Instruction::Not(dst, src) => {
                [77, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::NotImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [78, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }
            Instruction::And(dst, lhs, rhs) => {
                [79, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::AndImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [80, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Or(dst, lhs, rhs) => {
                [81, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::OrImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [82, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Xor(dst, lhs, rhs) => {
                [83, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::XorImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [84, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::ShiftLeft(dst, lhs, rhs) => {
                [85, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::ShiftLeftImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [86, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::ShiftRight(dst, lhs, rhs) => {
                [87, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::ShiftRightImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [88, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }

            Instruction::Jump(reg) => {
                [89, *reg, 0, 0, 0, 0, 0, 0]
            }
            Instruction::JumpImmediate(addr) => {
                let addr = addr.to_le_bytes();
                [90, 0, 0, addr[0], addr[1], addr[2], addr[3], 0]
            }
            Instruction::JumpNotZero(reg, dst) => {
                [91, *reg, *dst, 0, 0, 0, 0, 0]
            }
            Instruction::JumpNotZeroImmediate(reg, addr) => {
                let addr = addr.to_le_bytes();
                [92, *reg, 0, addr[0], addr[1], addr[2], addr[3], 0]
            }

            Instruction::Move(dst, src) => {
                [93, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::MoveImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [94, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }

            Instruction::Push(reg) => {
                [95, *reg, 0, 0, 0, 0, 0, 0]
            }
            Instruction::PushImmediate(val) => {
                let val = val.to_le_bytes();
                [96, 0, 0, val[0], val[1], val[2], val[3], 0]
            }
            Instruction::Pop(reg) => {
                [97, *reg, 0, 0, 0, 0, 0, 0]
            }

            Instruction::Store(dst, src) => {
                let dst = dst.to_le_bytes();
                [98, dst[0], dst[1], dst[2], dst[3], *src, 0, 0]
            }
            Instruction::Load(dst, src) => {
                [99, *dst, *src, 0, 0, 0, 0, 0]
            }

            Instruction::Call(address) => {
                let address = address.to_le_bytes();
                [100, address[0], address[1], address[2], address[3], 0, 0, 0]
            }
            Instruction::Return => {
                [101, 0, 0, 0, 0, 0, 0, 0]
            }
        }
    }

    pub fn from_bfo_bytes(bytes: [u8; INSTRUCTION_SIZE as usize]) -> Instruction {
        match bytes[0] {
            0 => { // Instruction::Nop
                Instruction::Nop
            }
            1 => { // Instruction::Add
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Add(dist_reg, reg_lhs, reg_rhs)
            }
            2 => { // Instruction::AddImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::AddImmediate(dist_reg, reg_lhs, bits)
            }
            3 => { // Instruction::Sub
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Sub(dist_reg, reg_lhs, reg_rhs)
            }
            4 => { // Instruction::SubImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SubImmediate(dist_reg, reg_lhs, bits)
            }
            5 => { // Instruction::Mul
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Mul(dist_reg, reg_lhs, reg_rhs)
            }
            6 => { // Instruction::MulImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::MulImmediate(dist_reg, reg_lhs, bits)
            }
            7 => { // Instruction::Div
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Div(dist_reg, reg_lhs, reg_rhs)
            }
            8 => { // Instruction::DivImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::DivImmediate(dist_reg, reg_lhs, bits)
            }
            9 => { // Instruction::Mod
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Mod(dist_reg, reg_lhs, reg_rhs)
            }
            10 => { // Instruction::ModImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::ModImmediate(dist_reg, reg_lhs, bits)
            }
            11 => { // Instruction::DivMod
                let (dist_reg1, dist_reg2, reg_lhs, reg_rhs) = get_rrrr(&bytes,1);
                Instruction::DivMod(dist_reg1, dist_reg2, reg_lhs, reg_rhs)
            }
            12 => { // Instruction::DivModImmediate
                let (dist_reg1, dist_reg2, reg_lhs, bits) = get_rrrb(&bytes,1);
                Instruction::DivModImmediate(dist_reg1, dist_reg2, reg_lhs, bits)
            }

            13 => { // Instruction::GreaterThan
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::GreaterThan(dist_reg, reg_lhs, reg_rhs)
            }
            14 => { // Instruction::GreaterThanImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::GreaterThanImmediate(dist_reg, reg_lhs, bits)
            }
            15 => { // Instruction::LessThan
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::LessThan(dist_reg, reg_lhs, reg_rhs)
            }
            16 => { // Instruction::LessThanImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::LessThanImmediate(dist_reg, reg_lhs, bits)
            }
            17 => { // Instruction::GreaterThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::GreaterThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            18 => { // Instruction::GreaterThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::GreaterThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            19 => { // Instruction::LessThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::LessThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            20 => { // Instruction::LessThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::LessThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            21 => { // Instruction::Equal
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Equal(dist_reg, reg_lhs, reg_rhs)
            }
            22 => { // Instruction::EqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::EqualImmediate(dist_reg, reg_lhs, bits)
            }
            23 => { // Instruction::NotEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::NotEqual(dist_reg, reg_lhs, reg_rhs)
            }
            24 => { // Instruction::NotEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::NotEqualImmediate(dist_reg, reg_lhs, bits)
            }

            25 => { // Instruction::FloatAdd
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatAdd(dist_reg, reg_lhs, reg_rhs)
            }
            26 => { // Instruction::FloatAddImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatAddImmediate(dist_reg, reg_lhs, bits)
            }
            27 => { // Instruction::FloatSub
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatSub(dist_reg, reg_lhs, reg_rhs)
            }
            28 => { // Instruction::FloatSubImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatSubImmediate(dist_reg, reg_lhs, bits)
            }
            29 => { // Instruction::FloatMul
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatMul(dist_reg, reg_lhs, reg_rhs)
            }
            30 => { // Instruction::FloatMulImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatMulImmediate(dist_reg, reg_lhs, bits)
            }
            31 => { // Instruction::FloatDiv
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatDiv(dist_reg, reg_lhs, reg_rhs)
            }
            32 => { // Instruction::FloatDivImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatDivImmediate(dist_reg, reg_lhs, bits)
            }
            33 => { // Instruction::FloatMod
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatMod(dist_reg, reg_lhs, reg_rhs)
            }
            34 => { // Instruction::FloatModImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatModImmediate(dist_reg, reg_lhs, bits)
            }
            35 => { // Instruction::FloatDivMod
                let (dist_reg1, dist_reg2, reg_lhs, reg_rhs) = get_rrrr(&bytes,1);
                Instruction::FloatDivMod(dist_reg1, dist_reg2, reg_lhs, reg_rhs)
            }
            36 => { // Instruction::FloatDivModImmediate
                let (dist_reg1, dist_reg2, reg_lhs, bits) = get_rrrb(&bytes,1);
                Instruction::FloatDivModImmediate(dist_reg1, dist_reg2, reg_lhs, bits)
            }

            37 => { // Instruction::FloatGreaterThan
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatGreaterThan(dist_reg, reg_lhs, reg_rhs)
            }
            38 => { // Instruction::FloatGreaterThanImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatGreaterThanImmediate(dist_reg, reg_lhs, bits)
            }
            39 => { // Instruction::FloatLessThan
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatLessThan(dist_reg, reg_lhs, reg_rhs)
            }
            40 => { // Instruction::FloatLessThanImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatLessThanImmediate(dist_reg, reg_lhs, bits)
            }
            41 => { // Instruction::FloatGreaterThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatGreaterThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            42 => { // Instruction::FloatGreaterThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatGreaterThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            43 => { // Instruction::FloatLessThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatLessThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            44 => { // Instruction::FloatLessThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatLessThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            45 => { // Instruction::FloatEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatEqual(dist_reg, reg_lhs, reg_rhs)
            }
            46 => { // Instruction::FloatEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatEqualImmediate(dist_reg, reg_lhs, bits)
            }
            47 => { // Instruction::FloatNotEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::FloatNotEqual(dist_reg, reg_lhs, reg_rhs)
            }
            48 => { // Instruction::FloatNotEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::FloatNotEqualImmediate(dist_reg, reg_lhs, bits)
            }
            49 => { // Instruction::FloatNegate
                let (dist_reg, reg_lhs) = get_rr(&bytes,1);
                Instruction::FloatNegate(dist_reg, reg_lhs)
            }
            50 => { // Instruction::FloatNegateImmediate
                let (dist_reg, bits) = get_rb(&bytes,1);
                Instruction::FloatNegateImmediate(dist_reg, bits)
            }

            51 => { // Instruction::SignedAdd
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedAdd(dist_reg, reg_lhs, reg_rhs)
            }
            52 => { // Instruction::SignedAddImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedAddImmediate(dist_reg, reg_lhs, bits)
            }
            53 => { // Instruction::SignedSub
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedSub(dist_reg, reg_lhs, reg_rhs)
            }
            54 => { // Instruction::SignedSubImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedSubImmediate(dist_reg, reg_lhs, bits)
            }
            55 => { // Instruction::SignedMul
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedMul(dist_reg, reg_lhs, reg_rhs)
            }
            56 => { // Instruction::SignedMulImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedMulImmediate(dist_reg, reg_lhs, bits)
            }
            57 => { // Instruction::SignedDiv
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedDiv(dist_reg, reg_lhs, reg_rhs)
            }
            58 => { // Instruction::SignedDivImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedDivImmediate(dist_reg, reg_lhs, bits)
            }
            59 => { // Instruction::SignedMod
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedMod(dist_reg, reg_lhs, reg_rhs)
            }
            60 => { // Instruction::SignedModImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedModImmediate(dist_reg, reg_lhs, bits)
            }
            61 => { // Instruction::SignedDivMod
                let (dist_reg1, dist_reg2, reg_lhs, reg_rhs) = get_rrrr(&bytes,1);
                Instruction::SignedDivMod(dist_reg1, dist_reg2, reg_lhs, reg_rhs)
            }
            62 => { // Instruction::SignedDivModImmediate
                let (dist_reg1, dist_reg2, reg_lhs, bits) = get_rrrb(&bytes,1);
                Instruction::SignedDivModImmediate(dist_reg1, dist_reg2, reg_lhs, bits)
            }

            63 => { // Instruction::SignedGreaterThan
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedGreaterThan(dist_reg, reg_lhs, reg_rhs)
            }
            64 => { // Instruction::SignedGreaterThanImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedGreaterThanImmediate(dist_reg, reg_lhs, bits)
            }
            65 => { // Instruction::SignedLessThan
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedLessThan(dist_reg, reg_lhs, reg_rhs)
            }
            66 => { // Instruction::SignedLessThanImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedLessThanImmediate(dist_reg, reg_lhs, bits)
            }
            67 => { // Instruction::SignedGreaterThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedGreaterThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            68 => { // Instruction::SignedGreaterThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedGreaterThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            69 => { // Instruction::SignedLessThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedLessThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            70 => { // Instruction::SignedLessThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedLessThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            71 => { // Instruction::SignedEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedEqual(dist_reg, reg_lhs, reg_rhs)
            }
            72 => { // Instruction::SignedEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedEqualImmediate(dist_reg, reg_lhs, bits)
            }
            73 => { // Instruction::SignedNotEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::SignedNotEqual(dist_reg, reg_lhs, reg_rhs)
            }
            74 => { // Instruction::SignedNotEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::SignedNotEqualImmediate(dist_reg, reg_lhs, bits)
            }
            75 => { // Instruction::SignedNegate
                let (dist_reg, reg_lhs) = get_rr(&bytes,1);
                Instruction::SignedNegate(dist_reg, reg_lhs)
            }
            76 => { // Instruction::SignedNegateImmediate
                let (dist_reg, bits) = get_rb(&bytes,1);
                Instruction::SignedNegateImmediate(dist_reg, bits)
            }

            77 => { // Instruction::Not
                let (dist_reg, reg_lhs) = get_rr(&bytes,1);
                Instruction::Not(dist_reg, reg_lhs)
            }
            78 => { // Instruction::NotImmediate
                let (dist_reg, bits) = get_rb(&bytes,1);
                Instruction::NotImmediate(dist_reg, bits)
            }
            79 => { // Instruction::And
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::And(dist_reg, reg_lhs, reg_rhs)
            }
            80 => { // Instruction::AndImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::AndImmediate(dist_reg, reg_lhs, bits)
            }
            81 => { // Instruction::Or
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Or(dist_reg, reg_lhs, reg_rhs)
            }
            82 => { // Instruction::OrImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::OrImmediate(dist_reg, reg_lhs, bits)
            }
            83 => { // Instruction::Xor
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::Xor(dist_reg, reg_lhs, reg_rhs)
            }
            84 => { // Instruction::XorImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::XorImmediate(dist_reg, reg_lhs, bits)
            }
            85 => { // Instruction::ShiftLeft
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::ShiftLeft(dist_reg, reg_lhs, reg_rhs)
            }
            86 => { // Instruction::ShiftLeftImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::ShiftLeftImmediate(dist_reg, reg_lhs, bits)
            }
            87 => { // Instruction::ShiftRight
                let (dist_reg, reg_lhs, reg_rhs) = get_rrr(&bytes,1);
                Instruction::ShiftRight(dist_reg, reg_lhs, reg_rhs)
            }
            88 => { // Instruction::ShiftRightImmediate
                let (dist_reg, reg_lhs, bits) = get_rrb(&bytes,1);
                Instruction::ShiftRightImmediate(dist_reg, reg_lhs, bits)
            }

            89 => { // Instruction::Jump
                let (reg) = get_r(&bytes,1);
                Instruction::Jump(reg)
            }
            90 => { // Instruction::JumpImmediate
                let (bits) = get_b(&bytes,1);
                Instruction::JumpImmediate(bits)
            }
            91 => { // Instruction::JumpNotZero
                let (reg_cnd, reg_jmp) = get_rr(&bytes,1);
                Instruction::JumpNotZero(reg_cnd, reg_jmp)
            }
            92 => { // Instruction::JumpNotZeroImmediate
                let (reg_cnd, bits) = get_rb(&bytes,1);
                Instruction::JumpNotZeroImmediate(reg_cnd, bits)
            }

            93 => { // Instruction::Move
                let (dist_reg, reg_lhs) = get_rr(&bytes,1);
                Instruction::Move(dist_reg, reg_lhs)
            }
            94 => { // Instruction::MoveImmediate
                let (dist_reg, bits) = get_rb(&bytes,1);
                Instruction::MoveImmediate(dist_reg, bits)
            }

            95 => { // Instruction::Push
                let (reg) = get_r(&bytes,1);
                Instruction::Push(reg)
            }
            96 => { // Instruction::PushImmediate
                let (bits) = get_b(&bytes,1);
                Instruction::PushImmediate(bits)
            }
            97 => { // Instruction::Pop
                let (reg) = get_r(&bytes,1);
                Instruction::Pop(reg)
            }

            98 => { // Instruction::Store
                let (store_address, value_reg) = get_ar(&bytes,1);
                Instruction::Store(store_address, value_reg)
            }
            99 => { // Instruction::Load
                let (dist_reg, source_reg) = get_rr(&bytes,1);
                Instruction::Load(dist_reg, source_reg)
            }

            100 => { // Instruction::Call
                let (address) = get_b(&bytes,1);
                Instruction::Call(address)
            }
            101 => { // Instruction::Return
                Instruction::Return
            }
            _ => unimplemented!(),
        }
    }
}