use std::fs;
use num::bigint::BigInt;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong!");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let mut input: Vec<i32> = s.split(',').map(|num| num.parse::<i32>().unwrap()).collect();
    for _ in 0..80 {
        let num_zeroes = input.iter().filter(|num| **num == 0 ).fold(0, |acc, _| acc + 1 );
        input = input.iter().map(|num| num - 1).collect();
        input = input.iter().map(|num| {
            if *num == -1 {
                6
            } else {
                *num
            }
        }).collect();
        for _ in 0..num_zeroes {
            input.push(8);
        }
    }
    
    println!("{}", input.len());
}

fn second_challenge(s: &String) {
    let input: Vec<u32> = s.split(',').map(|num| num.parse::<u32>().unwrap()).collect();
    let mut zeroes = input.iter().filter(|num| **num == 0).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut ones = input.iter().filter(|num| **num == 1).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut twos = input.iter().filter(|num| **num == 2).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut threes = input.iter().filter(|num| **num == 3).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut fours = input.iter().filter(|num| **num == 4).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut fives = input.iter().filter(|num| **num == 5).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut sixes = input.iter().filter(|num| **num == 6).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut sevens = input.iter().filter(|num| **num == 7).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);
    let mut eights = input.iter().filter(|num| **num == 8).fold(BigInt::new(num::bigint::Sign::NoSign, vec![0]), |acc, _| acc + 1);

    for _ in 0..256 {
        let z1 = zeroes.clone();
        let z2 = zeroes.clone();
        let o = ones.clone();
        let t = twos.clone();
        let th = threes.clone();
        let fo = fours.clone();
        let fi = fives.clone();
        let s = sixes.clone();
        let se = sevens.clone();
        let e = eights.clone();

        eights = z1;
        sevens = e;
        sixes = se + z2;
        fives = s;
        fours = fi;
        threes = fo;
        twos = th;
        ones = t;
        zeroes = o;
    }

    println!("{}", zeroes + ones + twos + threes + fours + fives + sixes + sevens + eights);
}