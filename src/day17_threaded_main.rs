use threadpool::ThreadPool;
use std::time::Instant;
use std::sync::mpsc::channel;
use std::fs;

// on my computer this came with the fastest time.
// but fun fact, this runs slower than the single threaded counterpart.
// I can chalk this up to a coupel things:
// 1) this is using message passing, a shared vec may be faster, but I really didn't want to deal with boundaries
// 2) there is a sort at the end that is not needed for the single threaded version. This loses about 200ms
// 3) this is a purely CPU bound task that has no sign of letting up, so switching threads may not be the best and they are very short lived threads at that
const WORKERS: usize = 4;
const NUM_JOBS: usize = 4000000;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
}

fn first_challenge(s: &String) {
    let now = Instant::now();
    let pool = ThreadPool::new(WORKERS);
    let coords = s.split(':').collect::<Vec<&str>>()[1].trim().split(", ").collect::<Vec<&str>>();
    let x: Vec<i32> = coords[0].get(2..).unwrap().split("..").map(|item| i32::from_str_radix(item, 10).unwrap()).collect();
    let y: Vec<i32> = coords[1].get(2..).unwrap().split("..").map(|item| i32::from_str_radix(item, 10).unwrap()).collect();
    let (tx, rx) = channel();
    for xx in -1000..=1000 {
        for yy in -1000..=1000 {
            let tx = tx.clone();
            let xx = xx.clone();
            let yy = yy.clone();
            let y = y.clone();
            let x = x.clone();
            pool.execute(move|| {
                let mut x_step = xx;
                let mut x_pos = 0;
                let mut y_step = yy;
                let mut y_pos = 0;
                while true {
                    if x_pos > x[1] {
                        break;
                    }
                    if y_pos < y[0] {
                        break;
                    }
                    if x_pos >= x[0] && x_pos <= x[1] && y_pos >= y[0] && y_pos <= y[1] {
                        // correct_pairs.push((xx, yy));
                        tx.send(Some((xx, yy)));
                    }

                    y_pos += y_step;
                    y_step -= 1;
                    if x_step > 0 {
                        x_pos += x_step;
                        x_step -= 1;
                    }
                }
                tx.send(None);
            });
        }
    }
    let mut correct_pairs: Vec<(i32, i32)> = rx.iter().take(NUM_JOBS).filter(|item| item.is_some()).map(|item| item.unwrap()).collect();
    let max_y = correct_pairs.iter().map(|item| item.1).max().unwrap();
    let mut start = 0;
    let mut stepper = max_y;
    while stepper != 0 {
        start += stepper;
        stepper -= 1;
    }
    println!("{}", start);
    correct_pairs.sort();
    correct_pairs.dedup();
    println!("{:?}", correct_pairs.len());
    println!("{}", now.elapsed().as_millis())
}
