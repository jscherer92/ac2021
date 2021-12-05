use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong!");
    first_challenge(&contents);
}

fn first_challenge(s: &String) {
    let mut max_x = 0;
    let mut max_y = 0;
    let coords: Vec<((i32, i32), (i32, i32))> = s.lines().map(|item| {
        let coords: Vec<(i32, i32)> = item.split_whitespace().filter_map(|i| {
            match i == "->" {
                true => {
                    None
                }
                false => {
                    let split_item: Vec<&str> = i.split(',').collect();
                    let x_coord = split_item[0].parse::<i32>().unwrap();
                    let y_coord = split_item[1].parse::<i32>().unwrap();
                    if x_coord > max_x {
                        max_x = x_coord;
                    }
                    if y_coord > max_y {
                        max_y = y_coord;
                    }
                    Some((x_coord, y_coord))
                }
            }
        }).collect();
        (coords[0], coords[1])
    }).collect();
    let mut coordinate_system: Vec<Vec<i32>> = (0..=max_x).map(|_num| vec![0; (max_y + 1) as usize]).collect();
    coords.iter().for_each(|&line| {
        // out x coords are the same, so we interpolate over our y coords
        if line.0.0 == line.1.0 {
            match line.0.1 > line.1.1 {
                true => {
                    (line.1.1..=line.0.1).for_each(|coord| coordinate_system[line.0.0 as usize][coord as usize] += 1 );
                }
                false => {
                    (line.0.1..=line.1.1).for_each(|coord| coordinate_system[line.0.0 as usize][coord as usize] += 1 );
                }
            }
        } else if line.0.1 == line.1.1 { // otherwise interpolate over our x coords
            match line.0.0 > line.1.0 {
                true => {
                    (line.1.0..=line.0.0).for_each(|coord| coordinate_system[coord as usize][line.0.1 as usize] += 1 );
                },
                false => {
                    (line.0.0..=line.1.0).for_each(|coord| coordinate_system[coord as usize][line.0.1 as usize] += 1 );
                }
            }
        } else { // we get a clue that it can only be 45 degree angles which means this is actually easier diagonal code
            match line.0.0 > line.1.0 {
                true => {
                    let go_backwards = line.1.1 > line.0.1;
                    for (pos, coord) in (line.1.0..=line.0.0).enumerate() {
                        if go_backwards {
                            coordinate_system[coord as usize][line.1.1 as usize - pos] += 1;
                        } else {
                            coordinate_system[coord as usize][line.1.1 as usize + pos] += 1;
                        }
                    }
                },
                false => {
                    let go_backwards = line.0.1 > line.1.1;
                    for (pos, coord) in (line.0.0..=line.1.0).enumerate() {
                        if go_backwards {
                            coordinate_system[coord as usize][line.0.1 as usize - pos] += 1;
                        } else {
                            coordinate_system[coord as usize][line.0.1 as usize + pos] += 1;
                        }
                    }
                }
            }
        }
    });

    let fin_answer: i32 = coordinate_system.iter().flatten().map(|num| if *num > 1 { 1 } else { 0 }).sum();
    println!("{}", fin_answer);
}
