use pest::Parser;
use bffcore::constants::instructions::Instruction;


#[derive(Parser)]
#[grammar = "bff_asm.pest"]
struct BffAsmBareParser;


pub struct BffAsmParser {
    pub instructions: Vec<Instruction>
}

impl BffAsmParser {
    pub fn new() -> BffAsmParser {
        BffAsmParser {
            instructions: vec![]
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<(), String> {
        let pairs = BffAsmBareParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
        for pair in pairs {
            match pair.as_rule() {
                Rule::move_ => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let value = inner_rules.next().unwrap();
                    match value.as_rule() {
                        Rule::operation_type => {
                            let op_type = value.as_str();
                            match op_type {
                                "u" => {
                                    // unsigned immediate
                                    let value = inner_rules.next().unwrap().as_str().parse::<u32>().unwrap();
                                    self.instructions.push(Instruction::MoveImmediate(dst_reg, value));
                                }
                                "s" => {
                                    // signed immediate
                                    let value = inner_rules.next().unwrap().as_str().parse::<i32>().unwrap();
                                    self.instructions.push(Instruction::MoveImmediate(dst_reg, value as u32));
                                }
                                "f" => {
                                    // float immediate
                                    let value = inner_rules.next().unwrap().as_str().parse::<f32>().unwrap();
                                    self.instructions.push(Instruction::MoveImmediate(dst_reg, value.to_bits()));
                                }
                                _ => unreachable!()
                            }
                        }
                        Rule::register => {
                            let src_reg = value.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::Move(dst_reg, src_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::equal => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.instructions.push(Instruction::EqualImmediate(dst_reg, lhs_reg, value.to_bits()));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::EqualImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.instructions.push(Instruction::EqualImmediate(dst_reg, lhs_reg, value as u32));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::Equal(dst_reg, lhs_reg, rhs_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::not_equal => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.instructions.push(Instruction::NotEqualImmediate(dst_reg, lhs_reg, value.to_bits()));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::NotEqualImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.instructions.push(Instruction::NotEqualImmediate(dst_reg, lhs_reg, value as u32));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::NotEqual(dst_reg, lhs_reg, rhs_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::not => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let src = inner_rules.next().unwrap();
                    match src.as_rule() {
                        Rule::register => {
                            let src_reg = src.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::Not(dst_reg, src_reg));
                        }
                        Rule::float => {
                            let value = src.as_str().parse::<f32>().unwrap();
                            self.instructions.push(Instruction::NotImmediate(dst_reg, value.to_bits()));
                        }
                        Rule::unsigned => {
                            let value = src.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::NotImmediate(dst_reg, value));
                        }
                        Rule::signed => {
                            let value = src.as_str().parse::<i32>().unwrap();
                            self.instructions.push(Instruction::NotImmediate(dst_reg, value as u32));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::and => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.instructions.push(Instruction::AndImmediate(dst_reg, lhs_reg, value.to_bits()));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::AndImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.instructions.push(Instruction::AndImmediate(dst_reg, lhs_reg, value as u32));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::And(dst_reg, lhs_reg, rhs_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::or => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.instructions.push(Instruction::OrImmediate(dst_reg, lhs_reg, value.to_bits()));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::OrImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.instructions.push(Instruction::OrImmediate(dst_reg, lhs_reg, value as u32));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::Or(dst_reg, lhs_reg, rhs_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::xor => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::float => {
                            let value = rhs.as_str().parse::<f32>().unwrap();
                            self.instructions.push(Instruction::XorImmediate(dst_reg, lhs_reg, value.to_bits()));
                        }
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::XorImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            let value = rhs.as_str().parse::<i32>().unwrap();
                            self.instructions.push(Instruction::XorImmediate(dst_reg, lhs_reg, value as u32));
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::Xor(dst_reg, lhs_reg, rhs_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::shift_left => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::ShiftLeftImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            panic!("Signed shift left is not supported")
                        }
                        Rule::float => {
                            panic!("Float shift left is not supported")
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::ShiftLeft(dst_reg, lhs_reg, rhs_reg));
                        }
                        path => unreachable!("{:?}", path)
                    }
                }
                Rule::shift_right => {
                    let mut inner_rules = pair.into_inner();
                    let dst_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let lhs_reg = inner_rules.next().unwrap().as_str()[3..].parse::<u8>().unwrap();
                    let rhs = inner_rules.next().unwrap();

                    match rhs.as_rule() {
                        Rule::unsigned => {
                            let value = rhs.as_str().parse::<u32>().unwrap();
                            self.instructions.push(Instruction::ShiftRightImmediate(dst_reg, lhs_reg, value));
                        }
                        Rule::signed => {
                            panic!("Signed shift right is not supported")
                        }
                        Rule::float => {
                            panic!("Float shift right is not supported")
                        }
                        Rule::register => {
                            let rhs_reg = rhs.as_str()[3..].parse::<u8>().unwrap();
                            self.instructions.push(Instruction::ShiftRight(dst_reg, lhs_reg, rhs_reg));
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
        Ok(())
    }
}