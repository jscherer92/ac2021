use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
}

fn first_challenge(s: &String) {
    let coords = s.split(':').collect::<Vec<&str>>()[1].trim().split(", ").collect::<Vec<&str>>();
    let x: Vec<i32> = coords[0].get(2..).unwrap().split("..").map(|item| i32::from_str_radix(item, 10).unwrap()).collect();
    let y: Vec<i32> = coords[1].get(2..).unwrap().split("..").map(|item| i32::from_str_radix(item, 10).unwrap()).collect();

    let mut correct_pairs: Vec<(i32, i32)> = Vec::new();
    for xx in -1000..=1000 {
        for yy in -1000..=1000 {
            let mut x_step = xx.clone();
            let mut x_pos = 0;
            let mut y_step = yy.clone();
            let mut y_pos = 0;
            while true {
                if x_pos > x[1] {
                    break;
                }
                if y_pos < y[0] {
                    break;
                }
                if x_pos >= x[0] && x_pos <= x[1] && y_pos >= y[0] && y_pos <= y[1] {
                    correct_pairs.push((xx, yy));
                }

                y_pos += y_step;
                y_step -= 1;
                if x_step > 0 {
                    x_pos += x_step;
                    x_step -= 1;
                }
            }
        }
    }
    let max_y = correct_pairs.iter().map(|item| item.1).max().unwrap();
    let mut start = 0;
    let mut stepper = max_y;
    while stepper != 0 {
        start += stepper;
        stepper -= 1;
    }
    println!("{}", start);
    correct_pairs.dedup();
    println!("{:?}", correct_pairs.len());
}
