use std::fs;

const one_num_segments: usize = 2;
const four_num_segements: usize = 4;
const seven_num_segements: usize = 3;
const eight_num_segements: usize = 7;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let input_output: Vec<Vec<&str>> = s.lines().map(|line| line.split('|').collect()).collect();
    let output: Vec<&str> = input_output.iter().map(|line| line[1].split_whitespace()).flatten().collect();
    let num = output.iter().map(|&item| {
        item.chars().fold(vec![false; 7], |mut acc, c| {
            match c {
                'a' => acc[0] = true,
                'b' => acc[1] = true,
                'c' => acc[2] = true,
                'd' => acc[3] = true,
                'e' => acc[4] = true,
                'f' => acc[5] = true,
                'g' => acc[6] = true,
                _ => println!("{} not correct input!", c)
            };
            acc
        }).iter().filter(|&&item|  item).collect::<Vec<&bool>>().len()
    }).filter(|&item| item == one_num_segments || item == four_num_segements || item == seven_num_segements || item == eight_num_segements).collect::<Vec<usize>>().len();
    println!("{}", num);
}

fn second_challenge(s: &String) {
    let input_output: Vec<Vec<&str>> = s.lines().map(|line| line.split('|').collect()).collect();
    // for each item in the list we need to figure out the sequence
    let fin: i32 = input_output.iter().map(|list| {
        let f = list[0].split_whitespace().map(|item| {
            item.chars().fold((0, 0), |mut acc, c| {
                match c {
                    'a' => acc.1 |= 1,
                    'b' => acc.1 |= 2,
                    'c' => acc.1 |= 4,
                    'd' => acc.1 |= 8,
                    'e' => acc.1 |= 16,
                    'f' => acc.1 |= 32,
                    'g' => acc.1 |= 64,
                    _ => acc.1 |= 128
                };
                acc.0 += 1;
                acc
            })
        }).collect::<Vec<(i32, i32)>>();

        let one = f.iter().find(|item| item.0 == 2).unwrap().1;
        let seven = f.iter().find(|item| item.0 == 3).unwrap().1;
        let eight = f.iter().find(|item| item.1 == 127).unwrap().1;
        let four = f.iter().find(|item| item.0 == 4).unwrap().1;

        let six_vecs: Vec<i32> = f.iter().filter(|item| item.0 == 6).map(|item| item.1).collect::<Vec<i32>>();
        let five_vecs: Vec<i32> = f.iter().filter(|item| item.0 == 5).map(|item| item.1).collect::<Vec<i32>>();

        let tt: i32 = seven ^ one;
        let tl_mm: i32 = four ^ one;
        let mm: i32 = tl_mm ^ (tl_mm & *six_vecs.iter().find(|&&item| tl_mm & item != tl_mm).unwrap());
        let tl: i32 = tl_mm ^ mm;
        let nine_check:i32 = tt | tl_mm | one;
        let bb: i32 =  *six_vecs.iter().find(|&item| nine_check & item == nine_check).unwrap() ^ nine_check;
        let five_check: i32 = tl_mm | bb | tt;
        let br: i32 =  *five_vecs.iter().find(|&item| five_check & item == five_check).unwrap() ^ five_check;
        let nine_check: i32 = five_check | br;
        let tr: i32 = nine_check ^ *six_vecs.iter().find(|&&item| (four | seven | bb) == item).unwrap();
        let final_check: i32 = nine_check | tr;
        let bl: i32 = eight ^ final_check;

        let zero_bin = tt | tr | br | bb | bl | tl; 
        let one_bin = one;
        let two_bin = tt| tr | mm | bl | bb;
        let three_bin = tt | tr | mm | br| bb;
        let four_bin = four;
        let five_bin = tt | tl | mm | br | bb;
        let six_bin = tt | tl | mm | br | bb | bl;
        let seven_bin = seven;
        // don't necessarily need to do this, but oh well
        let eight_bin = tt | tr | br | bb | bl | tl | mm;
        let nine_bin = tt | tr | br | bb | mm | tl;

        // now we have which letters go to which array segment, we go through the output and figure out the number
        let output = list[1].split_whitespace().map(|item| {
            item.chars().fold(0, |acc, c| {
                match c {
                    'a' => acc | 1,
                    'b' => acc | 2,
                    'c' => acc | 4,
                    'd' => acc | 8,
                    'e' => acc | 16,
                    'f' => acc | 32,
                    'g' => acc | 64,
                    _ => 0
                }
            })
        }).collect::<Vec<i32>>();

        let wat = output.iter().map(|&s| {
            if s == zero_bin {
                0
            } else if s == one_bin {
                1
            } else if s == two_bin {
                2
            } else if s == three_bin {
                3
            } else if s == four_bin {
                4
            } else if s == five_bin {
                5
            } else if s == six_bin {
                6
            } else if s == seven_bin {
                7
            } else if s == eight_bin {
                8
            } else if s == nine_bin {
                9
            } else {
                panic!("something went wrong!");
            }
        }).collect::<Vec<i32>>();

        let mut countdown = wat.len() - 1;
        let power: i32 = 10;
        let num = wat.iter().fold(0, |mut acc, item| {
            acc += item * power.pow(countdown as u32);
            if countdown > 0 {
                countdown -= 1;
            }
            acc
        });
        num
    }).sum();
    println!("{}", fin);
}
