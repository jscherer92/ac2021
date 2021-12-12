use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    conn: Vec<String>, // graph types are not simple in Rust do to borrow lifetimes. I don't want to bring in a separate library to do this, so I am just going to use indices into a stored vector
    loc: usize,
}

fn first_challenge(s: &String) {
    let mut caveLocLookup: HashMap<&str, usize> = HashMap::new();
    let mut caves: Vec<Cave> = Vec::new();

    s.lines().for_each(|line| {
        let path: Vec<&str> = line.split('-').collect();
        if !caveLocLookup.contains_key(path[0]) {
            let item = Cave {
                name: String::from(path[0]),
                conn: Vec::new(),
                loc: caves.len()
            };
            caves.push(item);
            caveLocLookup.insert(path[0], caves.len() - 1);
        }
      
        if !caveLocLookup.contains_key(path[1]) {
            let item = Cave {
                name: String::from(path[1]),
                conn: Vec::new(),
                loc: caves.len()
            };
            caves.push(item);
            caveLocLookup.insert(path[1], caves.len() - 1);
        }

        if path[1] != "start" && path[0] != "end" { 
            caves[caveLocLookup[path[0]]].conn.push(String::from(path[1]));
        }
        if path[0] != "start" && path[1] != "end" {
            caves[caveLocLookup[path[1]]].conn.push(String::from(path[0]));
        }
    });

    let start = &caves[caveLocLookup["start"]];
    let num = visit(&start, &caves, String::from(""));
    println!("{}", num);
}

fn visit(cave: &Cave, caves: &Vec<Cave>, s: String) -> usize {
    let is_small = cave.name == cave.name.to_lowercase();
    let visited: Vec<&str> = s.split('-').collect();
    if cave.name == "end" {
        if visited.iter().filter(|&&item| item != "start" && item != "end" && item.to_lowercase() == item).collect::<Vec<&&str>>().len() > 0 {
            return 1
        } else {
            return 0
        }
    } else if is_small && visited.contains(&cave.name.as_str()) && s != "start" {
        return 0
    } else {
        let mut co = 0;
        for (_, c) in cave.conn.iter().enumerate() {
            let item = caves.iter().find(|i| &i.name == c).unwrap();
            let num = visit(&item, &caves, String::from(s.clone() + "-" + &cave.name));
            co += num;
        }
        return co;
    }
}

fn second_challenge(s: &String) {
    let mut caveLocLookup: HashMap<&str, usize> = HashMap::new();
    let mut caves: Vec<Cave> = Vec::new();

    s.lines().for_each(|line| {
        let path: Vec<&str> = line.split('-').collect();
        if !caveLocLookup.contains_key(path[0]) {
            let item = Cave {
                name: String::from(path[0]),
                conn: Vec::new(),
                loc: caves.len()
            };
            caves.push(item);
            caveLocLookup.insert(path[0], caves.len() - 1);
        }
      
        if !caveLocLookup.contains_key(path[1]) {
            let item = Cave {
                name: String::from(path[1]),
                conn: Vec::new(),
                loc: caves.len()
            };
            caves.push(item);
            caveLocLookup.insert(path[1], caves.len() - 1);
        }

        if path[1] != "start" && path[0] != "end" { 
            caves[caveLocLookup[path[0]]].conn.push(String::from(path[1]));
        }
        if path[0] != "start" && path[1] != "end" {
            caves[caveLocLookup[path[1]]].conn.push(String::from(path[0]));
        }
    });

    let start = &caves[caveLocLookup["start"]];
    let num = visit2(&start, &caves, String::from(""));
    println!("{}", num);
}

fn visit2(cave: &Cave, caves: &Vec<Cave>, s: String) -> usize {
    let new_str = String::from(s.clone() + "-" + &cave.name);
    let is_small = cave.name == cave.name.to_lowercase();
    let mut visited: Vec<String> = new_str.split('-').map(|item| String::from(item)).collect();
    visited.remove(0);
    let hash = HashMap::new();
    let map = visited.iter().fold(hash, |mut acc: HashMap<&str, usize>, item: &String| {
        // we only want to count the single caves not the large caves
        if (*item).to_lowercase() == *item {
           *acc.entry(item.as_str()).or_insert(0) += 1;
        }
        acc
    });
    let val: Vec<&usize> = map.values().filter(|item| **item >= 2 ).collect::<Vec<&usize>>();
    if cave.name == "end" {
        if visited.iter().filter(|&item| item.as_str() != "start" && item.as_str() != "end" && item.as_str().to_lowercase() == item.as_str()).collect::<Vec<&String>>().len() > 0 {
            return 1
        } else {
            return 0
        }  
    } else if val.len() > 1 || (val.len() == 1 && val[0] > &2) {
        return 0
    } else if is_small && val.len() > 1 && s != "start" {
        return 0
    } else {
        let mut co = 0;
        for (_, c) in cave.conn.iter().enumerate() {
            let item = caves.iter().find(|i| &i.name == c).unwrap();
            let num = visit2(&item, &caves, String::from(s.clone() + "-" + &cave.name));
            co += num;
        }
        return co;
    }
}