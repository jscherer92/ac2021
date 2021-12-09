use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let lines: Vec<&str> = s.lines().collect();
    let line_length = lines.len();
    let data: Vec<Vec<usize>> = lines.iter().map(|line| line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect()).collect();
    let mut num_low_points: (usize, usize) = (0, 0);
    for i in 0..line_length {
        for j in 0..data[i].len() {
            if i == 0 {
                if j == 0 {
                    if data[i+1][j] > data[i][j] && data[i][j+1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }
                } else if j == data[i].len() - 1 {
                    if data[i+1][j] > data[i][j] && data[i][j-1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }
                } else {
                    if data[i+1][j] > data[i][j] && data[i][j-1] > data[i][j] && data[i][j+1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }
                }
            } else if i == line_length - 1 {
                if j == 0 {
                    if data[i-1][j] > data[i][j] && data[i][j+1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    } 
                } else if j == data[i].len() - 1 {
                    if data[i-1][j] > data[i][j] && data[i][j-1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }      
                } else {
                    if data[i-1][j] > data[i][j] && data[i][j+1] > data[i][j] && data[i][j-1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }
                }
            } else {
                if j == 0 {
                    if data[i+1][j] > data[i][j] && data[i-1][j] > data[i][j] && data[i][j+1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    } 
                } else if j == data[i].len() - 1 {
                    if data[i+1][j] > data[i][j] && data[i-1][j] > data[i][j] && data[i][j-1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }
                } else {
                    if data[i+1][j] > data[i][j] && data[i-1][j] > data[i][j] && data[i][j+1] > data[i][j] && data[i][j-1] > data[i][j] {
                        num_low_points.0 += 1;
                        num_low_points.1 += data[i][j] + 1;
                    }
                }
            }
        }
    }
    println!("{}", num_low_points.1);
}

fn second_challenge(s: &String) {
    let lines: Vec<&str> = s.lines().collect();
    let line_length = lines.len();
    let mut data: Vec<Vec<(usize, bool)>> = lines.iter().map(|line| {
        let mut v = line.chars().map(|c| (c.to_string().parse::<usize>().unwrap(), false)).collect::<Vec<(usize, bool)>>();
        v.insert(0, (9, false));
        v.push((9, false));
        v
    }).collect();
    data.insert(0, vec![(9, false); data[0].len()]);
    data.push(vec![(9, false); data[0].len()]);

    let mut basins: Vec<usize> = Vec::new();
    for i in 0..line_length {
        for j in 0..data[i].len() {
            if i == 0 {
                if j == 0 {
                    if !data[i][j].1 && data[i+1][j].0 > data[i][j].0 && data[i][j+1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let right_check = check_surrounding(&mut data, i, j + 1);
                        let down_check = check_surrounding(&mut data, i + 1, j);
                        basins.push(right_check + down_check + 1);
                    }
                } else if j == data[i].len() - 1 {
                    if !data[i][j].1 && data[i+1][j].0 > data[i][j].0 && data[i][j-1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let left_check = check_surrounding(&mut data, i, j - 1);
                        let down_check = check_surrounding(&mut data, i + 1, j);
                        basins.push(left_check + down_check + 1);
                    }
                } else {
                    if !data[i][j].1 && data[i+1][j].0 > data[i][j].0 && data[i][j-1].0 > data[i][j].0 && data[i][j+1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let left_check = check_surrounding(&mut data, i, j - 1);
                        let right_check = check_surrounding(&mut data, i, j + 1);
                        let down_check = check_surrounding(&mut data, i + 1, j);
                        basins.push(left_check + right_check + down_check + 1);
                    }
                }
            } else if i == line_length - 1 {
                if j == 0 {
                    if !data[i][j].1 && data[i-1][j].0 > data[i][j].0 && data[i][j+1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let right_check = check_surrounding(&mut data, i, j + 1);
                        let up_check = check_surrounding(&mut data, i - 1, j);
                        basins.push(right_check + up_check);
                    } 
                } else if j == data[i].len() - 1 {
                    if !data[i][j].1 && data[i-1][j].0 > data[i][j].0 && data[i][j-1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let left_check = check_surrounding(&mut data, i, j - 1);
                        let up_check = check_surrounding(&mut data, i - 1, j);
                        basins.push(left_check + up_check + 1);
                    }      
                } else {
                    if !data[i][j].1 && data[i-1][j].0 > data[i][j].0 && data[i][j+1].0 > data[i][j].0 && data[i][j-1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let left_check = check_surrounding(&mut data, i, j - 1);
                        let right_check = check_surrounding(&mut data, i, j + 1);
                        let up_check = check_surrounding(&mut data, i - 1, j);
                        basins.push(left_check + right_check + up_check + 1);
                    }
                }
            } else {
                if j == 0 {
                    if !data[i][j].1 && data[i+1][j].0 > data[i][j].0 && data[i-1][j].0 > data[i][j].0 && data[i][j+1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let down_check = check_surrounding(&mut data, i + 1, j);
                        let right_check = check_surrounding(&mut data, i, j + 1);
                        let up_check = check_surrounding(&mut data, i - 1, j);
                        basins.push(down_check + right_check + up_check + 1);
                    } 
                } else if j == data[i].len() - 1 {
                    if !data[i][j].1 && data[i+1][j].0 > data[i][j].0 && data[i-1][j].0 > data[i][j].0 && data[i][j-1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let down_check = check_surrounding(&mut data, i + 1, j);
                        let left_check = check_surrounding(&mut data, i, j - 1);
                        let up_check = check_surrounding(&mut data, i - 1, j);
                        basins.push(down_check + left_check + up_check + 1);
                    }
                } else {
                    if !data[i][j].1 && data[i+1][j].0 > data[i][j].0 && data[i-1][j].0 > data[i][j].0 && data[i][j+1].0 > data[i][j].0 && data[i][j-1].0 > data[i][j].0 {
                        data[i][j].1 = true;
                        let down_check = check_surrounding(&mut data, i + 1, j);
                        let left_check = check_surrounding(&mut data, i, j - 1);
                        let up_check = check_surrounding(&mut data, i - 1, j);
                        let right_check = check_surrounding(&mut data, i, j + 1);
                        basins.push(down_check + left_check + up_check + right_check + 1);
                    }
                }
            }
        }
    }
    basins.sort();
    println!("{:?}", basins.iter().rev().take(3).fold(1, |acc, num| acc * num));
}

fn check_surrounding(map: &mut Vec<Vec<(usize, bool)>>, i: usize, j: usize) -> usize {
    println!("{} {}", i, j);
    if map[i][j].1 {
        0
    } else if i > map.len() || j > map[0].len() {
        0
    } else if map[i][j].0 == 9 {
        map[i][j].1 = true;
        0
    } else {
        map[i][j].1 = true;
        1 + check_surrounding(map, i + 1, j) + check_surrounding(map, i - 1, j) + check_surrounding(map, i, j + 1) + check_surrounding(map, i, j - 1)
    }
}
