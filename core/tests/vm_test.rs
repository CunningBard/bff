use bffcore;
use bffcore::constants::instructions::Instruction;
use bffcore::engine::virtual_machine::VirtualMachine;

#[test]
fn unsigned_add() {
    let lhs = 1;
    let rhs = 2;
    let expected = lhs + rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MoveImmediate(2, rhs),
        Instruction::Add(1, 1, 2),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_sub() {
    let lhs = 5;
    let rhs = 2;
    let expected = lhs - rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MoveImmediate(2, rhs),
        Instruction::Sub(1, 1, 2),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_mul() {
    let lhs = 1;
    let rhs = 2;
    let expected = lhs * rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MoveImmediate(2, rhs),
        Instruction::Mul(1, 1, 2),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_div() {
    let lhs = 34;
    let rhs = 2;
    let expected = lhs / rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MoveImmediate(2, rhs),
        Instruction::Div(1, 1, 2),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_mod() {
    let lhs = 34;
    let rhs = 2;
    let expected = lhs % rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MoveImmediate(2, rhs),
        Instruction::Mod(1, 1, 2),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_div_mod() {
    let lhs = 34;
    let rhs = 2;
    let expected_div = lhs / rhs;
    let expected_mod = lhs % rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MoveImmediate(2, rhs),
        Instruction::DivMod(3, 4, 1, 2),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[3], expected_div);
    assert_eq!(vm.registers[4], expected_mod);
}

#[test]
fn unsigned_add_imm() {
    let lhs = 1;
    let rhs = 2;
    let expected = lhs + rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::AddImmediate(1, 1,rhs),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_sub_imm() {
    let lhs = 5;
    let rhs = 2;
    let expected = lhs - rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::SubImmediate(1, 1, rhs),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_mul_imm() {
    let lhs = 1;
    let rhs = 2;
    let expected = lhs * rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::MulImmediate(1, 1, rhs),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_div_imm() {
    let lhs = 34;
    let rhs = 2;
    let expected = lhs / rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::DivImmediate(1, 1, rhs),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_mod_imm() {
    let lhs = 34;
    let rhs = 2;
    let expected = lhs % rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::ModImmediate(1, 1, rhs),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[1], expected);
}

#[test]
fn unsigned_div_mod_imm() {
    let lhs = 34;
    let rhs = 2;
    let expected_div = lhs / rhs;
    let expected_mod = lhs % rhs;

    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, lhs),
        Instruction::DivModImmediate(3, 4, 1, rhs),
    ]);
    vm.execute_instruction_list();
    assert_eq!(vm.registers[3], expected_div);
    assert_eq!(vm.registers[4], expected_mod);
}

#[test]
fn print_hello_world(){
    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::MoveImmediate(1, 72),
        Instruction::Store(0, 1, 3),
        Instruction::PushImmediate(1), // length
        Instruction::PushImmediate(0), // address
        Instruction::PushImmediate(1), // syscall number
        Instruction::SystemCall
    ]);
    vm.execute_instruction_list();
}