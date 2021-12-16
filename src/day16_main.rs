use std::fs;

#[derive(Debug)]
enum Type {
    Literal,
    Operator,
    OperatorLength,
    OperatorCount,
    Id,
    Looking
}


fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("something went wrong!");
    first_challenge(&contents);
}

fn get_input(s: &String) -> String {
    let mut full = String::from("");
    for c in s.chars() {
        match c {
            '0' => full.push_str("0000"),
            '1' => full.push_str("0001"),
            '2' => full.push_str("0010"),
            '3' => full.push_str("0011"),
            '4' => full.push_str("0100"),
            '5' => full.push_str("0101"),
            '6' => full.push_str("0110"),
            '7' => full.push_str("0111"),
            '8' => full.push_str("1000"),
            '9' => full.push_str("1001"),
            'A' => full.push_str("1010"),
            'B' => full.push_str("1011"),
            'C' => full.push_str("1100"),
            'D' => full.push_str("1101"),
            'E' => full.push_str("1110"),
            'F' => full.push_str("1111"),
            _ => panic!("not a char!")
        }
    }
    full
}

fn first_challenge(s: &String) {
    let item = get_input(s);

    let mut ids: Vec<u32> = Vec::new();
    let mut literals: Vec<u128> = Vec::new();
    let mut types: Vec<u32> = Vec::new();

    // there are better ways to do this, but I am very tired lol
    let mut curr_type = Type::Id;
    let mut curr_decode = String::from("");
    let mut curr_instruction_bit_count = 3;
    let mut curr_loc_in_instruction = 0;
    let mut curr_literal = String::from("");

    // when we are dealing with operator values
    let mut operator_stack: Vec<(Type, u32, u32, u32, u32, u32)> = Vec::new();

    for c in item.chars() {
        curr_decode.push(c);
        curr_loc_in_instruction += 1;
        // change state
        if curr_loc_in_instruction == curr_instruction_bit_count {
            match curr_type {
                Type::Id => {
                    ids.push(u32::from_str_radix(&curr_decode, 2).unwrap());
                    curr_decode = String::from("");
                    curr_type = Type::Looking;
                    curr_loc_in_instruction = 0;
                },
                Type::Looking => {
                    match curr_decode {
                        _ if curr_decode == "100" => {
                            curr_decode = String::from("");
                            curr_type = Type::Literal;
                            curr_instruction_bit_count = 5;
                            curr_loc_in_instruction = 0;
                        },
                        _ => {
                            types.push(u32::from_str_radix(&curr_decode, 2).unwrap());
                            curr_decode = String::from("");
                            curr_type = Type::Operator;
                            curr_loc_in_instruction = 0;
                            curr_instruction_bit_count = 1;
                        }
                    }
                },
                Type::Literal => {
                    if curr_decode.chars().nth(0).unwrap() == '0' {
                        let mut oper_finished = true;
                        let mut other_oper = 0;
                        curr_literal.push_str(curr_decode.get(1..).unwrap());
                        literals.push(u128::from_str_radix(&curr_literal, 2).unwrap());
                        while oper_finished {
                            if let Some(mut op) = operator_stack.pop() {
                                // check op type
                                if other_oper != 0 {
                                    op.5 += other_oper;
                                } else {
                                    op.5 += (3 + 3 + curr_literal.len() + (curr_literal.len() / 4)) as u32;
                                }
                                match op.0 {
                                    Type::OperatorLength => {
                                        if other_oper != 0 {
                                            op.2 += other_oper;
                                        } else {
                                            op.2 += (3 + 3 + curr_literal.len() + (curr_literal.len() / 4)) as u32;
                                        }
                                    },
                                    Type::OperatorCount => op.2 += 1,
                                    _ => panic!("Should not have reached this!")
                                }
                                op.4 += 1;
                                if op.2 != op.3 {
                                    operator_stack.insert(operator_stack.len(), op);
                                    oper_finished = false;
                                } else {
                                    other_oper = op.5;
                                    match op.1 {
                                        0 => {
                                            let mut sum = 0;
                                            while op.4 != 0 {
                                                sum += literals.pop().unwrap();
                                                op.4 -= 1;
                                            }
                                            literals.insert(literals.len(), sum);
                                        },
                                        1 => {
                                            let mut mult = 1;
                                            while op.4 != 0 {
                                                mult *= literals.pop().unwrap();
                                                op.4 -= 1;
                                            }
                                            literals.insert(literals.len(), mult);
                                        },
                                        2 => {
                                            let mut min: Vec<u128> = Vec::new();
                                            while op.4 != 0 {
                                                min.push(literals.pop().unwrap());
                                                op.4 -= 1;
                                            }
                                            literals.insert(literals.len(), min.iter().min().unwrap().clone());
                                        },
                                        3 => {
                                            let mut max: Vec<u128> = Vec::new();
                                            while op.4 != 0 {
                                                max.push(literals.pop().unwrap());
                                                op.4 -= 1;
                                            }
                                            literals.insert(literals.len(), max.iter().max().unwrap().clone());
                                        },
                                        5 => {
                                            if literals.pop().unwrap() < literals.pop().unwrap() {
                                                literals.push(1);
                                            } else {
                                                literals.push(0);
                                            }
                                        },
                                        6 => {
                                            if literals.pop().unwrap() > literals.pop().unwrap() {
                                                literals.push(1);
                                            } else {
                                                literals.push(0);
                                            }
                                        },
                                        7 => {
                                            if literals.pop().unwrap() == literals.pop().unwrap() {
                                                literals.push(1);
                                            } else {
                                                literals.push(0);
                                            }
                                        },
                                        _ => panic!("something went wrong!")
                                    }

                                    if operator_stack.is_empty() {
                                        break;
                                    }
                                }
                            }
                        }
                        curr_literal = String::from("");
                        
                        curr_type = Type::Id;
                        curr_decode = String::from("");
                        curr_instruction_bit_count = 3;
                        curr_loc_in_instruction = 0;
                    } else {
                        curr_literal.push_str(curr_decode.get(1..).unwrap());
                        curr_decode = String::from("");
                        curr_loc_in_instruction = 0;
                    }
                },
                Type::Operator => {
                    match curr_decode {
                        _ if curr_decode == "0" => {
                            curr_decode = String::from("");
                            curr_type = Type::OperatorLength;
                            curr_instruction_bit_count = 15;
                            curr_loc_in_instruction = 0;
                        },
                        _ => {
                            curr_decode = String::from("");
                            curr_type = Type::OperatorCount;
                            curr_instruction_bit_count = 11;
                            curr_loc_in_instruction = 0;
                        } 
                    }
                },
                Type::OperatorCount => {
                    operator_stack.push((Type::OperatorCount, types.pop().unwrap().clone(), 0, u32::from_str_radix(&curr_decode, 2).unwrap(), 0, 18));
                    curr_type = Type::Id;
                    curr_decode = String::from("");
                    curr_loc_in_instruction = 0;
                    curr_instruction_bit_count = 3;
                },
                Type::OperatorLength => {
                    operator_stack.push((Type::OperatorLength, types.pop().unwrap().clone(), 0, u32::from_str_radix(&curr_decode, 2).unwrap(), 0, 22));
                    curr_decode = String::from("");
                    curr_type = Type::Id;
                    curr_loc_in_instruction = 0;
                    curr_instruction_bit_count = 3;
                }
            }
        }
    }

    println!("{:?}", literals);
}