use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let mut input: Vec<i32> = s.split(',').map(|num| num.parse::<i32>().unwrap()).collect();
    input.sort();
    let mid = input[input.len() / 2];
    let fuel_used: i32 = input.iter().map(|num| (num - mid).abs()).sum();
    println!("{}", fuel_used);
}

fn second_challenge(s: &String) {
    let mut input: Vec<i32> = s.split(',').map(|num| num.parse::<i32>().unwrap()).collect();
    input.sort();
    let mean: i32 = (input.iter().sum::<i32>() as f32 / input.len() as f32).floor() as i32;
    let fuel_used: i32 = input.iter().map(|num| (1..=(num - mean).abs()).sum::<i32>() ).sum::<i32>();
    println!("{}", fuel_used);
}