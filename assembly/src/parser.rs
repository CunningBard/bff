use std::collections::HashMap;
use pest::Parser;
use bffcore::constants::instructions::Instruction;
use bffcore::constants::types::Address;


pub enum ParseIntermediate {
    Instruction(Instruction),
    Jump(String),
    Jnz(u8, String),
    Call(String),
}


#[derive(Parser)]
#[grammar = "bff_asm.pest"]
struct BffAsmBareParser;


pub struct BffAsmParser {
    pub intermediates: Vec<ParseIntermediate>,
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, usize>
}

macro_rules! get_register_number_from_next_pair {
    ($pairs: ident) => {
        $pairs.next().unwrap().as_str()[3..].parse::<u8>().unwrap()
    };
}

impl BffAsmParser {
    pub fn new() -> BffAsmParser {
        BffAsmParser {
            intermediates: vec![],
            instructions: vec![],
            labels: HashMap::new()
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<(), String> {
        let pairs = BffAsmBareParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
        for pair in pairs {
            match pair.as_rule() {
                Rule::label => {
                    let mut inner_rules = pair.into_inner();
                    let label = inner_rules.next().unwrap().as_str();
                    self.labels.insert(label.to_string(), self.intermediates.len() + 1);
                }
                Rule::move_ => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let value = inner_rules.next().unwrap();
                    match value.as_rule() {
                        Rule::operation_type => {
                            let op_type = value.as_str();
                            match op_type {
                                "u" => {
                                    // unsigned immediate
                                    let value = inner_rules.next().unwrap().as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::MoveImmediate(dst_reg, value)));
                                }
                                "s" => {
                                    // signed immediate
                                    let value = inner_rules.next().unwrap().as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::MoveImmediate(dst_reg, value as u32)));
                                }
                                "f" => {
                                    // float immediate
                                    let value = inner_rules.next().unwrap().as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::MoveImmediate(dst_reg, value.to_bits())));
                                }
                                _ => unreachable!()
                            }
                        }
                        Rule::register => {
                            let src_reg = value.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Move(dst_reg, src_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::equal => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::EqualImmediate(dst_reg, lhs_reg, value.to_bits())));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::EqualImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::EqualImmediate(dst_reg, lhs_reg, value as u32)));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Equal(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::not_equal => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotEqualImmediate(dst_reg, lhs_reg, value.to_bits())));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotEqualImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotEqualImmediate(dst_reg, lhs_reg, value as u32)));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotEqual(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::not => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let src = inner_rules.next().unwrap();
                    match src.as_rule() {
                        Rule::register => {
                            let src_reg = src.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Not(dst_reg, src_reg)));
                        }
                        Rule::float => {
                            let value = src.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotImmediate(dst_reg, value.to_bits())));
                        }
                        Rule::unsigned => {
                            let value = src.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotImmediate(dst_reg, value)));
                        }
                        Rule::signed => {
                            let value = src.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::NotImmediate(dst_reg, value as u32)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::and => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::AndImmediate(dst_reg, lhs_reg, value.to_bits())));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::AndImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::AndImmediate(dst_reg, lhs_reg, value as u32)));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::And(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::or => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::OrImmediate(dst_reg, lhs_reg, value.to_bits())));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::OrImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::OrImmediate(dst_reg, lhs_reg, value as u32)));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Or(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::xor => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::XorImmediate(dst_reg, lhs_reg, value.to_bits())));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::XorImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::XorImmediate(dst_reg, lhs_reg, value as u32)));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Xor(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::shift_left => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::ShiftLeftImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            panic!("Signed shift left is not supported")
                        }
                        Rule::float => {
                            panic!("Float shift left is not supported")
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::ShiftLeft(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::shift_right => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::ShiftRightImmediate(dst_reg, lhs_reg, value)));
                        }
                        Rule::signed => {
                            panic!("Signed shift right is not supported")
                        }
                        Rule::float => {
                            panic!("Float shift right is not supported")
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::ShiftRight(dst_reg, lhs_reg, rhs_reg)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::jump => {
                    let mut inner_rules = pair.into_inner();
                    let label = inner_rules.next().unwrap().as_str();
                    self.intermediates.push(ParseIntermediate::Jump(label.to_string()));
                }
                Rule::jnz => {
                    let mut inner_rules = pair.into_inner();
                    let register = get_register_number_from_next_pair!(inner_rules);
                    let label = inner_rules.next().unwrap().as_str();
                    self.intermediates.push(ParseIntermediate::Jnz(register, label.to_string()));
                }
                Rule::push => {
                    let mut inner_rules = pair.into_inner();
                    let value = inner_rules.next().unwrap();
                    match value.as_rule() {
                        Rule::register => {
                            let reg = value.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Push(reg)));
                        }
                        Rule::unsigned => {
                            let value = value.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::PushImmediate(value)));
                        }
                        Rule::signed => {
                            let value = value.as_str().parse::<i32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::PushImmediate(value as u32)));
                        }
                        Rule::float => {
                            let value = value.as_str().parse::<f32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::PushImmediate(value.to_bits())));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::pop => {
                    let mut inner_rules = pair.into_inner();
                    let reg = get_register_number_from_next_pair!(inner_rules);
                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Pop(reg)));
                }
                Rule::call => {
                    let mut inner_rules = pair.into_inner();
                    let label = inner_rules.next().unwrap().as_str();
                    self.intermediates.push(ParseIntermediate::Call(label.to_string()));
                }
                Rule::ret => {
                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Return));
                }
                Rule::syscall => {
                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SystemCall));
                }
                Rule::store => {
                    let mut inner_rules = pair.into_inner();
                    let dst = inner_rules.next().unwrap();
                    let src_reg = get_register_number_from_next_pair!(inner_rules);
                    let size = inner_rules.next().unwrap().as_str().parse::<u8>().unwrap();


                    match dst.as_rule(){
                        Rule::register => {
                            let dst_reg = dst.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Store(dst_reg, src_reg, size)));
                        }
                        Rule::unsigned => {
                            let dst = dst.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::DirectStore(dst, src_reg, size)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::load => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let src = inner_rules.next().unwrap();
                    let size = inner_rules.next().unwrap().as_str().parse::<u8>().unwrap();

                    match src.as_rule(){
                        Rule::register => {
                            let src_reg = src.as_str()[3..].parse::<u8>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::Load(dst_reg, src_reg, size)));
                        }
                        Rule::unsigned => {
                            let src = src.as_str().parse::<u32>().unwrap();
                            self.intermediates.push(ParseIntermediate::Instruction(Instruction::DirectLoad(dst_reg, src, size)));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::add => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::AddImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed add is not supported")
                                }
                                Rule::float => {
                                    panic!("Float add is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Add(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedAddImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedAddImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float add is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedAdd(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned add is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed add is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatAddImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatAdd(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::sub => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SubImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed sub is not supported")
                                }
                                Rule::float => {
                                    panic!("Float sub is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Sub(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedSubImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedSubImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float sub is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedSub(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned sub is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed sub is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatSubImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatSub(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::mul => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::MulImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed mul is not supported")
                                }
                                Rule::float => {
                                    panic!("Float mul is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Mul(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedMulImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedMulImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float mul is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedMul(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned mul is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed mul is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatMulImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatMul(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::div => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::DivImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed div is not supported")
                                }
                                Rule::float => {
                                    panic!("Float div is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Div(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedDivImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedDivImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float div is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedDiv(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned div is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed div is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatDivImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatDiv(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::mod_ => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::ModImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed mod is not supported")
                                }
                                Rule::float => {
                                    panic!("Float mod is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::Mod(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedModImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedModImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float mod is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedMod(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => {
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned mod is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed mod is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatModImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatMod(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::divmod => {
                    let mut inner_rules = pair.into_inner();
                    let div_dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let mod_dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => { // Unsigned
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::DivModImmediate(div_dst_reg, mod_dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed divmod is not supported")
                                }
                                Rule::float => {
                                    panic!("Float divmod is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::DivMod(div_dst_reg, mod_dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => { // Signed
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedDivModImmediate(div_dst_reg, mod_dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedDivModImmediate(div_dst_reg, mod_dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float divmod is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedDivMod(div_dst_reg, mod_dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => { // Float
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned divmod is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed divmod is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatDivModImmediate(div_dst_reg, mod_dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatDivMod(div_dst_reg, mod_dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::greater_than => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => { // Unsigned
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::GreaterThanImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed greater than is not supported")
                                }
                                Rule::float => {
                                    panic!("Float greater than is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::GreaterThan(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => { // Signed
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedGreaterThanImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedGreaterThanImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float greater than is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedGreaterThan(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => { // Float
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned greater than is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed greater than is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatGreaterThanImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatGreaterThan(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::less_than => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => { // Unsigned
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::LessThanImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed less than is not supported")
                                }
                                Rule::float => {
                                    panic!("Float less than is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::LessThan(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => { // Signed
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedLessThanImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedLessThanImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float less than is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedLessThan(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => { // Float
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned less than is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed less than is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatLessThanImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatLessThan(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::greater_than_or_equal => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => { // Unsigned
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::GreaterThanOrEqualImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed greater than or equal is not supported")
                                }
                                Rule::float => {
                                    panic!("Float greater than or equal is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::GreaterThanOrEqual(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => { // Signed
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedGreaterThanOrEqualImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedGreaterThanOrEqualImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float greater than or equal is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedGreaterThanOrEqual(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => { // Float
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned greater than or equal is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed greater than or equal is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatGreaterThanOrEqualImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatGreaterThanOrEqual(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::less_than_or_equal => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let lhs_reg = get_register_number_from_next_pair!(inner_rules);
                    let rhs = inner_rules.next().unwrap();

                    match op_type {
                        "u" => { // Unsigned
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::LessThanOrEqualImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    panic!("Signed less than or equal is not supported")
                                }
                                Rule::float => {
                                    panic!("Float less than or equal is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::LessThanOrEqual(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "s" => { // Signed
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    let value = rhs.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedLessThanOrEqualImmediate(dst_reg, lhs_reg, value)));
                                }
                                Rule::signed => {
                                    let value = rhs.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedLessThanOrEqualImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float less than or equal is not supported")
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedLessThanOrEqual(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => { // Float
                            match rhs.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned less than or equal is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed less than or equal is not supported")
                                }
                                Rule::float => {
                                    let value = rhs.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatLessThanOrEqualImmediate(dst_reg, lhs_reg, value as u32)));
                                }
                                Rule::register => {
                                    let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatLessThanOrEqual(dst_reg, lhs_reg, rhs_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::negate => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = get_register_number_from_next_pair!(inner_rules);
                    let op_type = inner_rules.next().unwrap().as_str();
                    let val = inner_rules.next().unwrap();

                    match op_type {
                        "u" => { // Unsigned
                            panic!("Unsigned negate is not supported")
                        }
                        "s" => { // Signed
                            match val.as_rule() {
                                Rule::unsigned => {
                                    let value = val.as_str().parse::<u32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedNegateImmediate(dst_reg, value)));
                                }
                                Rule::signed => {
                                    let value = val.as_str().parse::<i32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedNegateImmediate(dst_reg, value as u32)));
                                }
                                Rule::float => {
                                    panic!("Float negate is not supported")
                                }
                                Rule::register => {
                                    let val_reg = val.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::SignedNegate(dst_reg, val_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        "f" => { // Float
                            match val.as_rule() {
                                Rule::unsigned => {
                                    panic!("Unsigned negate is not supported")
                                }
                                Rule::signed => {
                                    panic!("Signed negate is not supported")
                                }
                                Rule::float => {
                                    let value = val.as_str().parse::<f32>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatNegateImmediate(dst_reg, value as u32)));
                                }
                                Rule::register => {
                                    let val_reg = val.as_str()[3..].parse::<u8>().unwrap();
                                    self.intermediates.push(ParseIntermediate::Instruction(Instruction::FloatNegate(dst_reg, val_reg)));
                                }
                                path => unreachable!("{:?}", path)
                            }
                        }
                        path => unreachable!("{:?}", path)
                    }
                }

                Rule::EOI => {
                    break
                }
                path => unreachable!("{:?}", path)
            }
        }

        for intermediate in self.intermediates.iter() {
            match intermediate {
                ParseIntermediate::Instruction(instruction) => {
                    self.instructions.push(instruction.clone());
                }
                ParseIntermediate::Jump(jmp_label) => {
                    let jmp_instruction = Instruction::JumpImmediate(self.labels.get(&*jmp_label).unwrap().clone() as Address);
                    self.instructions.push(jmp_instruction);
                }
                ParseIntermediate::Jnz(reg, jmp_label) => {
                    let jmp_instruction = Instruction::JumpNotZeroImmediate(*reg, self.labels.get(&*jmp_label).unwrap().clone() as Address);
                    self.instructions.push(jmp_instruction);
                }
                ParseIntermediate::Call(jmp_label) => {
                    let jmp_instruction = Instruction::Call(self.labels.get(&*jmp_label).unwrap().clone() as Address);
                    self.instructions.push(jmp_instruction);
                }
            }
        }

        Ok(())
    }
}