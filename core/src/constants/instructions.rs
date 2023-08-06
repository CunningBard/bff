use crate::constants::constants::INSTRUCTION_SIZE;
use crate::constants::types::{Address, Bits, Byte, Register};

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

    Store(Address, Register, Byte), // load from heap, byte is the number of bytes to move i.e 1, 2, 4
    Load(Register, Register, Byte), // load from heap, byte is the number of bytes to move i.e 1, 2, 4
    // for compatibility reasons, the byte part in the Store/Load instruction is subtractive, meaning that 0 is 4 bytes, 2 is 2 bytes, and 3 is 1 byte
    // this is because the Store/Load instruction was originally designed to move 4 bytes at a time, and the byte part was added later on, the byte part was unused so back then this part was always 0

    Call(Address),
    Return,

    SystemCall
}

fn get_qb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u32, u8) {
    let address = u32::from_le_bytes([
        program[index],
        program[index + 1],
        program[index + 2],
        program[index + 3],
    ]);
    let reg = program[index + 4];

    (address, reg)
}

fn get_bd(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u8, u32) {
    let reg = program[index];
    let address = u32::from_le_bytes([
        program[index + 1],
        program[index + 2],
        program[index + 3],
        program[index + 4],
    ]);

    (reg, address)
}

fn get_b(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> u8 {
    program[index]
}

fn get_d(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> u32 {
    u32::from_le_bytes([
        program[index],
        program[index + 1],
        program[index + 2],
        program[index + 3],
    ])
}

fn get_bb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u8, u8) {
    let reg1 = program[index];
    let reg2 = program[index + 1];

    (reg1, reg2)
}

fn get_dbb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u32, u8, u8) {
    let address = u32::from_le_bytes([
        program[index],
        program[index + 1],
        program[index + 2],
        program[index + 3],
    ]);
    let reg1 = program[index + 4];
    let reg2 = program[index + 5];

    (address, reg1, reg2)
}

fn get_bbd(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u8, u8, u32) {
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

fn get_bbb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u8, u8, u8) {
    let reg1 = program[index];
    let reg2 = program[index + 1];
    let reg3 = program[index + 2];

    (reg1, reg2, reg3)
}

fn get_bbbb(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u8, u8, u8, u8) {
    let reg1 = program[index];
    let reg2 = program[index + 1];
    let reg3 = program[index + 2];
    let reg4 = program[index + 3];

    (reg1, reg2, reg3, reg4)
}

fn get_bbbd(program: &[u8; INSTRUCTION_SIZE as usize], index: usize) -> (u8, u8, u8, u32) {
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
            Instruction::FloatNegate(dst, src) => {
                [45, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::FloatNegateImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [46, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }

            Instruction::SignedAdd(dst, lhs, rhs) => {
                [47, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedAddImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [48, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedSub(dst, lhs, rhs) => {
                [49, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedSubImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [50, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedMul(dst, lhs, rhs) => {
                [51, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedMulImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [52, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedDiv(dst, lhs, rhs) => {
                [53, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedDivImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [54, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedMod(dst, lhs, rhs) => {
                [55, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedModImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [56, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedDivMod(dst, dst2, lhs, rhs) => {
                [57, *dst, *dst2, *lhs, *rhs, 0, 0, 0]
            }
            Instruction::SignedDivModImmediate(dst, dst2, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [58, *dst, *dst2, *lhs, 0, rhs[0], rhs[1], rhs[2]]
            }

            Instruction::SignedGreaterThan(dst, lhs, rhs) => {
                [59, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedGreaterThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [60, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedLessThan(dst, lhs, rhs) => {
                [61, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedLessThanImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [62, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedGreaterThanOrEqual(dst, lhs, rhs) => {
                [63, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedGreaterThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [64, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedLessThanOrEqual(dst, lhs, rhs) => {
                [65, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::SignedLessThanOrEqualImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [66, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::SignedNegate(dst, src) => {
                [67, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::SignedNegateImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [68, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }

            Instruction::Not(dst, src) => {
                [69, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::NotImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [70, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }
            Instruction::And(dst, lhs, rhs) => {
                [71, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::AndImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [72, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Or(dst, lhs, rhs) => {
                [73, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::OrImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [74, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::Xor(dst, lhs, rhs) => {
                [75, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::XorImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [76, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::ShiftLeft(dst, lhs, rhs) => {
                [77, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::ShiftLeftImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [78, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }
            Instruction::ShiftRight(dst, lhs, rhs) => {
                [79, *dst, *lhs, *rhs, 0, 0, 0, 0]
            }
            Instruction::ShiftRightImmediate(dst, lhs, rhs) => {
                let rhs = rhs.to_le_bytes();
                [80, *dst, *lhs, 0, rhs[0], rhs[1], rhs[2], rhs[3]]
            }

            Instruction::Jump(reg) => {
                [81, *reg, 0, 0, 0, 0, 0, 0]
            }
            Instruction::JumpImmediate(addr) => {
                let addr = addr.to_le_bytes();
                [82, 0, 0, addr[0], addr[1], addr[2], addr[3], 0]
            }
            Instruction::JumpNotZero(reg, dst) => {
                [83, *reg, *dst, 0, 0, 0, 0, 0]
            }
            Instruction::JumpNotZeroImmediate(reg, addr) => {
                let addr = addr.to_le_bytes();
                [84, *reg, 0, addr[0], addr[1], addr[2], addr[3], 0]
            }

            Instruction::Move(dst, src) => {
                [85, *dst, *src, 0, 0, 0, 0, 0]
            }
            Instruction::MoveImmediate(dst, src) => {
                let src = src.to_le_bytes();
                [86, *dst, 0, src[0], src[1], src[2], src[3], 0]
            }

            Instruction::Push(reg) => {
                [87, *reg, 0, 0, 0, 0, 0, 0]
            }
            Instruction::PushImmediate(val) => {
                let val = val.to_le_bytes();
                [88, 0, 0, val[0], val[1], val[2], val[3], 0]
            }
            Instruction::Pop(reg) => {
                [89, *reg, 0, 0, 0, 0, 0, 0]
            }

            Instruction::Store(dst, src, bytes_num) => {
                let dst = dst.to_le_bytes();
                [90, dst[0], dst[1], dst[2], dst[3], *src, *bytes_num, 0]
            }
            Instruction::Load(dst, src, byte_num) => {
                [91, *dst, *src, *byte_num, 0, 0, 0, 0]
            }

            Instruction::Call(address) => {
                let address = address.to_le_bytes();
                [92, address[0], address[1], address[2], address[3], 0, 0, 0]
            }
            Instruction::Return => {
                [93, 0, 0, 0, 0, 0, 0, 0]
            }
            Instruction::SystemCall => {
                [94, 0, 0, 0, 0, 0, 0, 0]
            }
        }
    }

    pub fn from_bfo_bytes(bytes: [u8; INSTRUCTION_SIZE as usize]) -> Instruction {
        match bytes[0] {
            0 => { // Instruction::Nop
                Instruction::Nop
            }
            1 => { // Instruction::Add
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Add(dist_reg, reg_lhs, reg_rhs)
            }
            2 => { // Instruction::AddImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::AddImmediate(dist_reg, reg_lhs, bits)
            }
            3 => { // Instruction::Sub
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Sub(dist_reg, reg_lhs, reg_rhs)
            }
            4 => { // Instruction::SubImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SubImmediate(dist_reg, reg_lhs, bits)
            }
            5 => { // Instruction::Mul
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Mul(dist_reg, reg_lhs, reg_rhs)
            }
            6 => { // Instruction::MulImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::MulImmediate(dist_reg, reg_lhs, bits)
            }
            7 => { // Instruction::Div
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Div(dist_reg, reg_lhs, reg_rhs)
            }
            8 => { // Instruction::DivImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::DivImmediate(dist_reg, reg_lhs, bits)
            }
            9 => { // Instruction::Mod
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Mod(dist_reg, reg_lhs, reg_rhs)
            }
            10 => { // Instruction::ModImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::ModImmediate(dist_reg, reg_lhs, bits)
            }
            11 => { // Instruction::DivMod
                let (dist_reg1, dist_reg2, reg_lhs, reg_rhs) = get_bbbb(&bytes, 1);
                Instruction::DivMod(dist_reg1, dist_reg2, reg_lhs, reg_rhs)
            }
            12 => { // Instruction::DivModImmediate
                let (dist_reg1, dist_reg2, reg_lhs, bits) = get_bbbd(&bytes, 1);
                Instruction::DivModImmediate(dist_reg1, dist_reg2, reg_lhs, bits)
            }

            13 => { // Instruction::GreaterThan
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::GreaterThan(dist_reg, reg_lhs, reg_rhs)
            }
            14 => { // Instruction::GreaterThanImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::GreaterThanImmediate(dist_reg, reg_lhs, bits)
            }
            15 => { // Instruction::LessThan
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::LessThan(dist_reg, reg_lhs, reg_rhs)
            }
            16 => { // Instruction::LessThanImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::LessThanImmediate(dist_reg, reg_lhs, bits)
            }
            17 => { // Instruction::GreaterThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::GreaterThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            18 => { // Instruction::GreaterThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::GreaterThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            19 => { // Instruction::LessThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::LessThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            20 => { // Instruction::LessThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::LessThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            21 => { // Instruction::Equal
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Equal(dist_reg, reg_lhs, reg_rhs)
            }
            22 => { // Instruction::EqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::EqualImmediate(dist_reg, reg_lhs, bits)
            }
            23 => { // Instruction::NotEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::NotEqual(dist_reg, reg_lhs, reg_rhs)
            }
            24 => { // Instruction::NotEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::NotEqualImmediate(dist_reg, reg_lhs, bits)
            }

            25 => { // Instruction::FloatAdd
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatAdd(dist_reg, reg_lhs, reg_rhs)
            }
            26 => { // Instruction::FloatAddImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatAddImmediate(dist_reg, reg_lhs, bits)
            }
            27 => { // Instruction::FloatSub
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatSub(dist_reg, reg_lhs, reg_rhs)
            }
            28 => { // Instruction::FloatSubImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatSubImmediate(dist_reg, reg_lhs, bits)
            }
            29 => { // Instruction::FloatMul
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatMul(dist_reg, reg_lhs, reg_rhs)
            }
            30 => { // Instruction::FloatMulImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatMulImmediate(dist_reg, reg_lhs, bits)
            }
            31 => { // Instruction::FloatDiv
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatDiv(dist_reg, reg_lhs, reg_rhs)
            }
            32 => { // Instruction::FloatDivImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatDivImmediate(dist_reg, reg_lhs, bits)
            }
            33 => { // Instruction::FloatMod
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatMod(dist_reg, reg_lhs, reg_rhs)
            }
            34 => { // Instruction::FloatModImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatModImmediate(dist_reg, reg_lhs, bits)
            }
            35 => { // Instruction::FloatDivMod
                let (dist_reg1, dist_reg2, reg_lhs, reg_rhs) = get_bbbb(&bytes, 1);
                Instruction::FloatDivMod(dist_reg1, dist_reg2, reg_lhs, reg_rhs)
            }
            36 => { // Instruction::FloatDivModImmediate
                let (dist_reg1, dist_reg2, reg_lhs, bits) = get_bbbd(&bytes, 1);
                Instruction::FloatDivModImmediate(dist_reg1, dist_reg2, reg_lhs, bits)
            }

            37 => { // Instruction::FloatGreaterThan
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatGreaterThan(dist_reg, reg_lhs, reg_rhs)
            }
            38 => { // Instruction::FloatGreaterThanImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatGreaterThanImmediate(dist_reg, reg_lhs, bits)
            }
            39 => { // Instruction::FloatLessThan
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatLessThan(dist_reg, reg_lhs, reg_rhs)
            }
            40 => { // Instruction::FloatLessThanImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatLessThanImmediate(dist_reg, reg_lhs, bits)
            }
            41 => { // Instruction::FloatGreaterThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatGreaterThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            42 => { // Instruction::FloatGreaterThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatGreaterThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            43 => { // Instruction::FloatLessThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::FloatLessThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            44 => { // Instruction::FloatLessThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::FloatLessThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            45 => { // Instruction::FloatNegate
                let (dist_reg, reg_lhs) = get_bb(&bytes, 1);
                Instruction::FloatNegate(dist_reg, reg_lhs)
            }
            46 => { // Instruction::FloatNegateImmediate
                let (dist_reg, bits) = get_bd(&bytes, 1);
                Instruction::FloatNegateImmediate(dist_reg, bits)
            }

            47 => { // Instruction::SignedAdd
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedAdd(dist_reg, reg_lhs, reg_rhs)
            }
            48 => { // Instruction::SignedAddImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedAddImmediate(dist_reg, reg_lhs, bits)
            }
            49 => { // Instruction::SignedSub
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedSub(dist_reg, reg_lhs, reg_rhs)
            }
            50 => { // Instruction::SignedSubImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedSubImmediate(dist_reg, reg_lhs, bits)
            }
            51 => { // Instruction::SignedMul
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedMul(dist_reg, reg_lhs, reg_rhs)
            }
            52 => { // Instruction::SignedMulImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedMulImmediate(dist_reg, reg_lhs, bits)
            }
            53 => { // Instruction::SignedDiv
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedDiv(dist_reg, reg_lhs, reg_rhs)
            }
            54 => { // Instruction::SignedDivImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedDivImmediate(dist_reg, reg_lhs, bits)
            }
            55 => { // Instruction::SignedMod
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedMod(dist_reg, reg_lhs, reg_rhs)
            }
            56 => { // Instruction::SignedModImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedModImmediate(dist_reg, reg_lhs, bits)
            }
            57 => { // Instruction::SignedDivMod
                let (dist_reg1, dist_reg2, reg_lhs, reg_rhs) = get_bbbb(&bytes, 1);
                Instruction::SignedDivMod(dist_reg1, dist_reg2, reg_lhs, reg_rhs)
            }
            58 => { // Instruction::SignedDivModImmediate
                let (dist_reg1, dist_reg2, reg_lhs, bits) = get_bbbd(&bytes, 1);
                Instruction::SignedDivModImmediate(dist_reg1, dist_reg2, reg_lhs, bits)
            }

            59 => { // Instruction::SignedGreaterThan
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedGreaterThan(dist_reg, reg_lhs, reg_rhs)
            }
            60 => { // Instruction::SignedGreaterThanImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedGreaterThanImmediate(dist_reg, reg_lhs, bits)
            }
            61 => { // Instruction::SignedLessThan
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedLessThan(dist_reg, reg_lhs, reg_rhs)
            }
            62 => { // Instruction::SignedLessThanImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedLessThanImmediate(dist_reg, reg_lhs, bits)
            }
            63 => { // Instruction::SignedGreaterThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedGreaterThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            64 => { // Instruction::SignedGreaterThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedGreaterThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            65 => { // Instruction::SignedLessThanOrEqual
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::SignedLessThanOrEqual(dist_reg, reg_lhs, reg_rhs)
            }
            66 => { // Instruction::SignedLessThanOrEqualImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::SignedLessThanOrEqualImmediate(dist_reg, reg_lhs, bits)
            }
            67 => { // Instruction::SignedNegate
                let (dist_reg, reg_lhs) = get_bb(&bytes, 1);
                Instruction::SignedNegate(dist_reg, reg_lhs)
            }
            68 => { // Instruction::SignedNegateImmediate
                let (dist_reg, bits) = get_bd(&bytes, 1);
                Instruction::SignedNegateImmediate(dist_reg, bits)
            }

            69 => { // Instruction::Not
                let (dist_reg, reg_lhs) = get_bb(&bytes, 1);
                Instruction::Not(dist_reg, reg_lhs)
            }
            70 => { // Instruction::NotImmediate
                let (dist_reg, bits) = get_bd(&bytes, 1);
                Instruction::NotImmediate(dist_reg, bits)
            }
            71 => { // Instruction::And
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::And(dist_reg, reg_lhs, reg_rhs)
            }
            72 => { // Instruction::AndImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::AndImmediate(dist_reg, reg_lhs, bits)
            }
            73 => { // Instruction::Or
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Or(dist_reg, reg_lhs, reg_rhs)
            }
            74 => { // Instruction::OrImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::OrImmediate(dist_reg, reg_lhs, bits)
            }
            75 => { // Instruction::Xor
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::Xor(dist_reg, reg_lhs, reg_rhs)
            }
            76 => { // Instruction::XorImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::XorImmediate(dist_reg, reg_lhs, bits)
            }
            77 => { // Instruction::ShiftLeft
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::ShiftLeft(dist_reg, reg_lhs, reg_rhs)
            }
            78 => { // Instruction::ShiftLeftImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::ShiftLeftImmediate(dist_reg, reg_lhs, bits)
            }
            79 => { // Instruction::ShiftRight
                let (dist_reg, reg_lhs, reg_rhs) = get_bbb(&bytes, 1);
                Instruction::ShiftRight(dist_reg, reg_lhs, reg_rhs)
            }
            80 => { // Instruction::ShiftRightImmediate
                let (dist_reg, reg_lhs, bits) = get_bbd(&bytes, 1);
                Instruction::ShiftRightImmediate(dist_reg, reg_lhs, bits)
            }

            81 => { // Instruction::Jump
                let reg = get_b(&bytes, 1);
                Instruction::Jump(reg)
            }
            82 => { // Instruction::JumpImmediate
                let bits = get_d(&bytes, 1);
                Instruction::JumpImmediate(bits)
            }
            83 => { // Instruction::JumpNotZero
                let (reg_cnd, reg_jmp) = get_bb(&bytes, 1);
                Instruction::JumpNotZero(reg_cnd, reg_jmp)
            }
            84 => { // Instruction::JumpNotZeroImmediate
                let (reg_cnd, bits) = get_bd(&bytes, 1);
                Instruction::JumpNotZeroImmediate(reg_cnd, bits)
            }

            85 => { // Instruction::Move
                let (dist_reg, source_reg) = get_bb(&bytes, 1);
                Instruction::Move(dist_reg, source_reg)
            }
            86 => { // Instruction::MoveImmediate
                let (dist_reg, bits) = get_bd(&bytes, 1);
                Instruction::MoveImmediate(dist_reg, bits)
            }

            87 => { // Instruction::Push
                let reg = get_b(&bytes, 1);
                Instruction::Push(reg)
            }
            88 => { // Instruction::PushImmediate
                let bits = get_d(&bytes, 1);
                Instruction::PushImmediate(bits)
            }
            89 => { // Instruction::Pop
                let reg = get_b(&bytes, 1);
                Instruction::Pop(reg)
            }

            90 => { // Instruction::Store
                let (store_address, value_reg, byte_num) = get_dbb(&bytes, 1);
                Instruction::Store(store_address, value_reg, byte_num)
            }
            91 => { // Instruction::Load
                let (dist_reg, source_reg, byte_num) = get_bbb(&bytes, 1);
                Instruction::Load(dist_reg, source_reg, byte_num)
            }

            92 => { // Instruction::Call
                let address = get_d(&bytes, 1);
                Instruction::Call(address)
            }
            93 => { // Instruction::Return
                Instruction::Return
            }
            94 => { // Instruction::SystemCall
                Instruction::SystemCall
            }
            _ => unimplemented!(),
        }
    }
}