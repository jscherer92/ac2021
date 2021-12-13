use std::fs;
use num_bigint::BigUint;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong!");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let input: Vec<&str> = s.lines().collect();
    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut largest_x = 0;
    let mut largest_y = 0;
    let mut folds: Vec<(bool, usize)> = Vec::new();
    let mut switch_enumerator = false;
    for (_, line) in input.iter().enumerate() {
        if line == &"" {
            switch_enumerator = true;
        } else if !switch_enumerator {
            let coord: Vec<&str> = line.split(",").collect();
            let x_coord = coord[0].parse::<usize>().unwrap();
            let y_coord = coord[1].parse::<usize>().unwrap();
            coords.push((x_coord, y_coord));
    
            if x_coord > largest_x {
                largest_x = x_coord;
            }
            if y_coord > largest_y {
                largest_y = y_coord;
            }
        } else {
            let fold: Vec<&str> = line.split(" ").collect();
            let dir = fold[2];
            let fin_dir: Vec<&str> = dir.split("=").collect();
            folds.push((fin_dir[0] == "x", fin_dir[1].parse::<usize>().unwrap()));
        }
    } 

    // now we are going to create numbers based on the y and x values
    let mut num_vecs: Vec<BigUint> = vec![BigUint::new(vec![0]); largest_y + 1];
    coords.iter().for_each(|set| {
        num_vecs[set.1] |= BigUint::new(vec![1]) << set.0;
    });

    let first_fold = folds[0];
    // y fold
    if !first_fold.0 {
        let piece_to_fold_up = &num_vecs[first_fold.1+1..=largest_y];
        let piece_to_fold_onto = &num_vecs[0..first_fold.1];
        let folded: Vec<(&BigUint, &BigUint)> = piece_to_fold_up.iter().zip(piece_to_fold_onto.iter().rev()).collect();
        let fin: Vec<BigUint> = folded.iter().map(|item| item.0 | item.1).collect();
        let num: u128 = fin.iter().map(|num| {
            let mut count: u128 = 0;
            let mut move_num = num.clone();
            while move_num.clone() > BigUint::from(0 as u32) {
                if move_num.clone() & BigUint::from(1 as u32) == BigUint::from(1 as u32) {
                    count += 1
                }
                move_num = move_num.clone() >> 1;
            }
            count
        }).sum();
        println!("{:?}", num);
    } else { // x fold
        let fold_loc = first_fold.1;
        let num: u128 = num_vecs.iter().map(|item| {
            let mut count: u128 = 0;
            let mut left_side_bit_num = fold_loc - 1;
            let mut right_side_big_num = fold_loc + 1;
            while left_side_bit_num >= 0 && right_side_big_num <= largest_x {
                if (item & (BigUint::from(1 as u32) << left_side_bit_num) != BigUint::from(0 as u32)) || (item & (BigUint::from(1 as u32) << right_side_big_num) != BigUint::from(0 as u32)) {
                    count += 1;
                } 
                if left_side_bit_num == 0 {
                    break;
                }
                left_side_bit_num -= 1;
                right_side_big_num += 1;
            }
            count
        }).sum();
        println!("{:?}", num);
    }
}

fn second_challenge(s: &String) {
    let input: Vec<&str> = s.lines().collect();
    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut largest_x = 0;
    let mut largest_y = 0;
    let mut folds: Vec<(bool, usize)> = Vec::new();
    let mut switch_enumerator = false;
    for (_, line) in input.iter().enumerate() {
        if line == &"" {
            switch_enumerator = true;
        } else if !switch_enumerator {
            let coord: Vec<&str> = line.split(",").collect();
            let x_coord = coord[0].parse::<usize>().unwrap();
            let y_coord = coord[1].parse::<usize>().unwrap();
            coords.push((x_coord, y_coord));
    
            if x_coord > largest_x {
                largest_x = x_coord;
            }
            if y_coord > largest_y {
                largest_y = y_coord;
            }
        } else {
            let fold: Vec<&str> = line.split(" ").collect();
            let dir = fold[2];
            let fin_dir: Vec<&str> = dir.split("=").collect();
            folds.push((fin_dir[0] == "x", fin_dir[1].parse::<usize>().unwrap()));
        }
    } 

    // now we are going to create numbers based on the y and x values
    let mut num_vecs: Vec<BigUint> = vec![BigUint::new(vec![0]); largest_y + 1];
    coords.iter().for_each(|set| {
        num_vecs[set.1] |= BigUint::new(vec![1]) << set.0;
    });
    
    for (_, fold) in folds.iter().enumerate() {
        if !fold.0 {
            let piece_to_fold_up = &num_vecs[fold.1+1..];
            let piece_to_fold_onto = &num_vecs[0..fold.1];
            let folded: Vec<(&BigUint, &BigUint)> = piece_to_fold_up.iter().zip(piece_to_fold_onto.iter().rev()).collect();
            let fin: Vec<BigUint> = folded.iter().map(|item| item.0 | item.1).collect();
            num_vecs = fin;
            largest_y /= 2;
        } else { // x fold
            let fold_loc = fold.1;
            num_vecs = num_vecs.iter().map(|item| {
                let mut count = BigUint::from(0 as u32);
                let mut left_side_bit_num = fold_loc - 1;
                let mut right_side_big_num = fold_loc + 1;
                while left_side_bit_num >= 0 && right_side_big_num <= largest_x {
                    if (item & (BigUint::from(1 as u32) << left_side_bit_num) != BigUint::from(0 as u32)) || (item & (BigUint::from(1 as u32) << right_side_big_num) != BigUint::from(0 as u32)) {
                        count |= BigUint::from(1 as u32) << left_side_bit_num;
                    } 
                    if left_side_bit_num == 0 {
                        break;
                    }
                    left_side_bit_num -= 1;
                    right_side_big_num += 1;
                }
                count
            }).collect();
            largest_x /= 2;
        }
    }
    num_vecs.iter().rev().for_each(|num| {
        let mut bit_loc = 0;
        while bit_loc <= largest_x {
            if num & (BigUint::from(1 as u32) << bit_loc) != BigUint::from(0 as u32) {
                print!("#");
            } else {
                print!(".");
            }
            bit_loc += 1;
        }
        println!();
    })
}
