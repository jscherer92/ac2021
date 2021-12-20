use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    // this problem can have a max amount at any point: 32 items 2 ^ 5

    let lines = s.lines().collect::<Vec<&str>>();

    let mut trees: Vec<Vec<(usize, i32)>> = Vec::new();

    trees = lines.iter().map(|line| {
        let mut curr_num: Vec<char> = Vec::new();
        let mut curr_depth: usize = 0;
        let mut tree: Vec<(usize, i32)> = Vec::new();
        for c in line.chars() {
            match c {
                '[' => {
                    curr_depth += 1;
                },
                ']' => {
                    if !curr_num.is_empty() {
                        tree.push((curr_depth, i32::from_str_radix(&String::from_iter(curr_num.clone()), 10).unwrap()));
                    }
                    curr_num = Vec::new();
                    curr_depth -= 1;
                },
                ',' => {
                    if !curr_num.is_empty() {
                        tree.push((curr_depth, i32::from_str_radix(&String::from_iter(curr_num.clone()), 10).unwrap()));
                    }
                    curr_num = Vec::new();
                },
                _ => {
                    curr_num.push(c);
                }
            }
        }
        tree
    }).collect::<Vec<Vec<(usize, i32)>>>();

    let mut working_tree = trees.remove(0);

    while !trees.is_empty() {
        let mut add_tree = trees.remove(0);
        println!();
        
        working_tree.append(&mut add_tree);

        working_tree = working_tree.iter().map(|ref mut item| {
            let mut n_item = item.clone();
            n_item.0 += 1;
            n_item
        }).collect::<Vec<(usize, i32)>>();

        let mut work_done = 0;

        // see if we need to do any work
        loop {
            for i in 0..working_tree.len() {
                // we need to explode
                if working_tree[i].0 == 5 {
                    let item = working_tree.remove(i);
                    let item2 = working_tree.remove(i);
                    working_tree.insert(i, (4, 0));
                    // look to our left
                    if i > 0 {
                        working_tree[i - 1].1 += item.1;
                    }
                    if i != working_tree.len() - 1 {
                        working_tree[i+1].1 += item2.1;
                    }
                    work_done +=1;
                    break;
                } 
            }
            if work_done != 0 {
                work_done = 0;
                continue;
            }
            for i in 0..working_tree.len() {
                if working_tree[i].1 > 9 {
                    let item = working_tree.remove(i);
                    let new_item = (item.0 + 1, item.1 / 2);
                    let new_item2 = (item.0 + 1, (item.1 as f32 / 2.0).ceil() as i32);
                    working_tree.insert(i, new_item2);
                    working_tree.insert(i, new_item);
                    work_done += 1;
                    break;
                }
            }

            if work_done == 0 {
                break;
            } else {
                work_done = 0;
            }
        }
    }

    // figure out the magnitude
    for depth in (1..5).rev() {
        while let Some(x) = working_tree.iter().position(|item| item.0 == depth) {
            let item = working_tree.remove(x);
            let next_item = working_tree.remove(x);
            working_tree.insert(x, (depth -1, item.1 * 3 + next_item.1 * 2));
        }
    }
}

fn second_challenge(s: &String) {
    let lines = s.lines().collect::<Vec<&str>>();

    let mut trees: Vec<Vec<(usize, i32)>> = Vec::new();

    trees = lines.iter().map(|line| {
        let mut curr_num: Vec<char> = Vec::new();
        let mut curr_depth: usize = 0;
        let mut tree: Vec<(usize, i32)> = Vec::new();
        for c in line.chars() {
            match c {
                '[' => {
                    curr_depth += 1;
                },
                ']' => {
                    if !curr_num.is_empty() {
                        tree.push((curr_depth, i32::from_str_radix(&String::from_iter(curr_num.clone()), 10).unwrap()));
                    }
                    curr_num = Vec::new();
                    curr_depth -= 1;
                },
                ',' => {
                    if !curr_num.is_empty() {
                        tree.push((curr_depth, i32::from_str_radix(&String::from_iter(curr_num.clone()), 10).unwrap()));
                    }
                    curr_num = Vec::new();
                },
                _ => {
                    curr_num.push(c);
                }
            }
        }
        tree
    }).collect::<Vec<Vec<(usize, i32)>>>();

    let mut sums: Vec<i32> = Vec::new();
    while !trees.is_empty() {
        let curr = trees.pop();
        for tree in trees.iter() {
            sums.push(sum_trees(curr.clone().unwrap(), tree.clone()));
            sums.push(sum_trees(tree.clone(), curr.clone().unwrap()));
        }
    }
    println!("{:?}", sums.iter().max());
}

fn sum_trees(ref first: Vec<(usize, i32)>, ref second: Vec<(usize, i32)>) -> i32 {
        let mut working_tree = first.clone();
        let mut add_tree = second.clone();
        
        working_tree.append(&mut add_tree);

        working_tree = working_tree.iter().map(|ref mut item| {
            let mut n_item = item.clone();
            n_item.0 += 1;
            n_item
        }).collect::<Vec<(usize, i32)>>();

        let mut work_done = 0;

        // see if we need to do any work
        loop {
            for i in 0..working_tree.len() {
                // we need to explode
                if working_tree[i].0 == 5 {
                    let item = working_tree.remove(i);
                    let item2 = working_tree.remove(i);
                    working_tree.insert(i, (4, 0));
                    // look to our left
                    if i > 0 {
                        working_tree[i - 1].1 += item.1;
                    }
                    if i != working_tree.len() - 1 {
                        working_tree[i+1].1 += item2.1;
                    }
                    work_done +=1;
                    break;
                } 
            }
            if work_done != 0 {
                work_done = 0;
                continue;
            }
            for i in 0..working_tree.len() {
                if working_tree[i].1 > 9 {
                    let item = working_tree.remove(i);
                    let new_item = (item.0 + 1, item.1 / 2);
                    let new_item2 = (item.0 + 1, (item.1 as f32 / 2.0).ceil() as i32);
                    working_tree.insert(i, new_item2);
                    working_tree.insert(i, new_item);
                    work_done += 1;
                    break;
                }
            }

            if work_done == 0 {
                break;
            } else {
                work_done = 0;
            }
        }

        // figure out the magnitude
    for depth in (1..5).rev() {
        while let Some(x) = working_tree.iter().position(|item| item.0 == depth) {
            let item = working_tree.remove(x);
            let next_item = working_tree.remove(x);
            working_tree.insert(x, (depth -1, item.1 * 3 + next_item.1 * 2));
        }
    }

    working_tree[0].1
}