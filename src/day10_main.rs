use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong!");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let points: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut stack: Vec<char> = Vec::new();
    let mut total = 0;

    s.lines().for_each(|line| {
        for c in line.chars() {
            match c {
                '(' => {
                    stack.push(c);
                },
                ')' => {
                    if *stack.last().unwrap() != '(' {
                        total += points[&c];
                        break;
                    }
                    stack.pop();
                },
                '[' => {
                    stack.push(c);
                },
                ']' => {
                    if *stack.last().unwrap() != '[' {
                        total += points[&c];
                        break;
                    }
                    stack.pop();
                },
                '{' => {
                    stack.push(c);
                },
                '}' => {
                    if *stack.last().unwrap() != '{' {
                        total += points[&c];
                        break;
                    }
                    stack.pop();
                },
                '<' => {
                    stack.push(c);
                },
                '>' => {
                    if *stack.last().unwrap() != '<' {
                        total += points[&c];
                        break;
                    }
                    stack.pop();
                },
                _ => panic!("unknown char!")
            }
        }
    });
    println!("{}", total);
}

fn second_challenge(s: &String) {
    let points: HashMap<char, u128> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut stack: Vec<char> = Vec::new();
    let mut total: Vec<u128> = Vec::new();

    s.lines().for_each(|line| {
        let mut broken_line = false;
        for c in line.chars() {
            match c {
                '(' => {
                    stack.push(c);
                },
                ')' => {
                    if *stack.last().unwrap() != '(' {
                        broken_line = true;
                        break;
                    }
                    stack.pop();
                },
                '[' => {
                    stack.push(c);
                },
                ']' => {
                    if *stack.last().unwrap() != '[' {
                        broken_line = true;
                        break;
                    }
                    stack.pop();
                },
                '{' => {
                    stack.push(c);
                },
                '}' => {
                    if *stack.last().unwrap() != '{' {
                        broken_line = true;
                        break;
                    }
                    stack.pop();
                },
                '<' => {
                    stack.push(c);
                },
                '>' => {
                    if *stack.last().unwrap() != '<' {
                        broken_line = true;
                        break;
                    }
                    stack.pop();
                },
                _ => panic!("unknown char!")
            }
        }
        // we have chars so now we need to work through them
        if stack.len() > 0 && !broken_line {
            let pts = stack.iter().rev().fold(0, |mut acc, val| {
                acc *= 5;
                acc += points[&val];
                acc
            });
            total.push(pts);
        }
        stack = Vec::new();
    });
    total.sort();
    let mid = total.len() / 2;
    println!("{}", total[mid]);
}
