use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challene(&contents);
    second_challenge(&contents);
} 


// check conditions:
// every 5 is true
// every num * 5 is true
// 1 - 7 - 13 - 19 - 25
// 21 - 17 - 13 - 9 - 5

fn first_challene(s: &String) {
    let parts: Vec<&str> = s.lines().collect();
    let mut part_iter = parts.iter();
    let mut bingo_cards: Vec<Vec<i32>> = Vec::new();
    let mut bingo_check_cards: Vec<Vec<bool>> = Vec::new();
    let bingo_input: &str = part_iter.next().unwrap();
    part_iter.next();
    let mut curr_card: Vec<i32> = Vec::new();

    for (i, line) in part_iter.enumerate() {
        match i % 6 == 5 { // reset the card
            true => {
                bingo_cards.push(curr_card);
                curr_card = Vec::new();
            },
            false => {
                curr_card.splice(..0, String::from(*line).split_whitespace().map(|num : &str| num.parse::<i32>().unwrap()));
            }
        } 
    }

    let input: Vec<i32> = String::from(bingo_input).split(',').map(|num| num.parse::<i32>().unwrap()).collect();
    let mut first_run: bool = true;
    let mut found_card = false;
    input.iter().for_each(|num| {
        let mut bingo_curr_loc = 0;
        bingo_cards.iter().for_each(|card| {
            let check_card: Vec<bool> = card.iter().map(|item| (item == num)).collect();
            if !first_run {
                let final_check_card: Vec<bool> = check_card.iter().zip(bingo_check_cards[bingo_curr_loc].iter()).map(|tuple| *tuple.0 || *tuple.1).collect();
                bingo_check_cards[bingo_curr_loc] = final_check_card;
            } else {
                bingo_check_cards.push(check_card);
            }
            bingo_curr_loc += 1;
        });
        // now check for winners. Should do this in the innter loop but oh well
        let mut curr_check_card: usize = 0;
        bingo_check_cards.iter().for_each(|check_card| {
            if check_card[0..=4].iter().all(|item| *item) || check_card[5..=9].iter().all(|item| *item) || check_card[10..=14].iter().all(|item| *item) || check_card[15..=19].iter().all(|item| *item) || check_card[20..=24].iter().all(|item| *item) {
                if !found_card {
                    found_card = true;
                    println!("Number found {}", num);
                    println!("Found card! {:?}", bingo_cards[curr_check_card]);
                    let sum = bingo_cards[curr_check_card].iter().zip(check_card.iter()).filter(|tuple| !*tuple.1).map(|tuple| *tuple.0).sum::<i32>();
                    println!("Solution {}", sum * num);
                }
            }
            if check_card.iter().step_by(5).all(|item| *item) || check_card.iter().skip(1).step_by(5).all(|item| *item) || check_card.iter().skip(2).step_by(5).all(|item| *item) || check_card.iter().skip(3).step_by(5).all(|item| *item) || check_card.iter().skip(4).step_by(5).all(|item| *item) {
                if !found_card {
                    found_card = true;
                    println!("Found by horizontal");
                    println!("Number found {}", num);
                    println!("Found card! {:?}", bingo_cards[curr_check_card]);
                    let sum = bingo_cards[curr_check_card].iter().zip(check_card.iter()).filter(|tuple| !*tuple.1).map(|tuple| *tuple.0).sum::<i32>();
                    println!("Solution {}", sum * num);
                }
            }
            curr_check_card += 1; 
        });
        first_run = false;
    });
}

fn second_challenge(s: &String) {
    let parts: Vec<&str> = s.lines().collect();
    let mut part_iter = parts.iter();
    let mut bingo_cards: Vec<Vec<i32>> = Vec::new();
    let mut bingo_check_cards: Vec<Vec<bool>> = Vec::new();
    let bingo_input: &str = part_iter.next().unwrap();
    part_iter.next();
    let mut curr_card: Vec<i32> = Vec::new();

    for (i, line) in part_iter.enumerate() {
        match i % 6 == 5 { // reset the card
            true => {
                bingo_cards.push(curr_card);
                curr_card = Vec::new();
            },
            false => {
                curr_card.splice(..0, String::from(*line).split_whitespace().map(|num : &str| num.parse::<i32>().unwrap()));
            }
        } 
    }

    let input: Vec<i32> = String::from(bingo_input).split(',').map(|num| num.parse::<i32>().unwrap()).collect();
    let mut first_run: bool = true;
    let mut final_check_card = false;
    let mut all_victory_cards: Vec<bool> = vec![false; bingo_cards.len()];
    input.iter().for_each(|num| {
        let mut bingo_curr_loc = 0;
        bingo_cards.iter().for_each(|card| {
            let check_card: Vec<bool> = card.iter().map(|item| (item == num)).collect();
            if !first_run {
                let final_check_card: Vec<bool> = check_card.iter().zip(bingo_check_cards[bingo_curr_loc].iter()).map(|tuple| *tuple.0 || *tuple.1).collect();
                bingo_check_cards[bingo_curr_loc] = final_check_card;
            } else {
                bingo_check_cards.push(check_card);
            }
            bingo_curr_loc += 1;
        });
        // now check for winners. Should do this in the innter loop but oh well
        let mut curr_check_card: usize = 0;
        bingo_check_cards.iter().for_each(|check_card| {
            if check_card[0..=4].iter().all(|item| *item) || check_card[5..=9].iter().all(|item| *item) || check_card[10..=14].iter().all(|item| *item) || check_card[15..=19].iter().all(|item| *item) || check_card[20..=24].iter().all(|item| *item) {
                all_victory_cards[curr_check_card] = true;
                if all_victory_cards.iter().all(|item| *item) && !final_check_card { 
                    println!("Number found {}", num);
                    println!("Found card! {:?}", bingo_cards[curr_check_card]);
                    let sum = bingo_cards[curr_check_card].iter().zip(check_card.iter()).filter(|tuple| !*tuple.1).map(|tuple| *tuple.0).sum::<i32>();
                    println!("Solution {}", sum * num);
                    final_check_card = true;
                }
            }
            if check_card.iter().step_by(5).all(|item| *item) || check_card.iter().skip(1).step_by(5).all(|item| *item) || check_card.iter().skip(2).step_by(5).all(|item| *item) || check_card.iter().skip(3).step_by(5).all(|item| *item) || check_card.iter().skip(4).step_by(5).all(|item| *item) {
                all_victory_cards[curr_check_card] = true;
                if all_victory_cards.iter().all(|item| *item) && !final_check_card { 
                    println!("Number found {}", num);
                    println!("Found card! {:?}", bingo_cards[curr_check_card]);
                    let sum = bingo_cards[curr_check_card].iter().zip(check_card.iter()).filter(|tuple| !*tuple.1).map(|tuple| *tuple.0).sum::<i32>();
                    println!("Solution {}", sum * num);
                    final_check_card = true;
                }
            }
            curr_check_card += 1; 
        });
        first_run = false;
    });
}