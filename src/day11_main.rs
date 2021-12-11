use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong!");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let mut arr: Vec<Vec<(bool, u128)>> = s.lines().map(|line| line.chars().map(|c| (false, c.to_digit(10).unwrap() as u128)).collect()).collect();
    let mut ans: u128 = 0;

    for _ in 0..100 {
        for i in 0..arr.len() {
            for j in 0.. arr[i].len() {
                arr[i][j].1 += 1;
            }
        }
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                if arr[i as usize][j as usize].1 > 9 {
                    flash(&mut arr, i as i16, j as i16);
                }
            }
        }
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                if arr[i][j].1 > 9 {
                    ans += 1;
                    arr[i][j].1 = 0;
                    arr[i][j].0 = false;
                }
            }
        }
    }
    println!("{}", ans);
    
}

fn second_challenge(s: &String) {
    let mut arr: Vec<Vec<(bool, u128)>> = s.lines().map(|line| line.chars().map(|c| (false, c.to_digit(10).unwrap() as u128)).collect()).collect();
    let mut step = 0;

    while true {
        for i in 0..arr.len() {
            for j in 0.. arr[i].len() {
                arr[i][j].1 += 1;
            }
        }
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                if arr[i as usize][j as usize].1 > 9 {
                    flash(&mut arr, i as i16, j as i16);
                }
            }
        }
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                if arr[i][j].1 > 9 {
                    arr[i][j].1 = 0;
                    arr[i][j].0 = false;
                }
            }
        }
        step += 1;
        if arr.iter().flatten().map(|item| item.1).sum::<u128>() == 0 {
            println!("{}", step);
            break;
        }
    }
}

fn flash(map: &mut Vec<Vec<(bool, u128)>>, i: i16, j: i16) {
    if i < 0 || j < 0 || i >= map.len() as i16 || j >= map[0].len() as i16 {
        return
    }
    map[i as usize][j as usize].1 += 1;
    if map[i as usize][j as usize].1 > 9 && !map[i as usize][j as usize].0 {
        map[i as usize][j as usize].0 = true;
        flash(map, i - 1, j);
        flash(map, i + 1, j);
        flash(map, i, j - 1);
        flash(map, i, j + 1);
        flash(map, i - 1, j - 1);
        flash(map, i - 1, j + 1);
        flash(map, i + 1, j - 1);
        flash(map, i + 1, j + 1);
    }
}