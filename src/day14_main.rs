use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn first_challenge(s: &String) {
    let mut lines = s.lines();
    let mut input = String::from(lines.next().unwrap());
    lines.next();
    let rules: HashMap<&str, char> = lines.map(|line| {
        let items: Vec<_> = line.split(" -> ").collect();
        (items[0], items[1].chars().take(1).collect::<Vec<char>>()[0])
    }).collect();
    
    for i in 0..10 {
        let mut next_input = String::from("");
        for j in 0..input.len()-1 {
            let item = &input[j..=j+1];
            next_input.push(rules[item]);
        }
        input = input.chars().zip(next_input.chars()).map(|item| [item.0, item.1]).flatten().collect::<String>() + String::from(input.pop().unwrap()).as_str();
    }

    let mut ans = HashMap::new();
    let fin = input.chars().fold(ans, |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
    println!("{:?}", fin);
}

fn second_challenge(s: &String) {
    let mut lines = s.lines();
    let mut input = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    let rules: HashMap<&str, char> = lines.map(|line| {
        let items: Vec<_> = line.split(" -> ").collect();
        (items[0], items[1].chars().take(1).collect::<Vec<char>>()[0])
    }).collect();

    let mut fin: HashMap<char, u128> = HashMap::new();
    for i in 0..input.len()-1 {
        let mut cache = HashMap::new();
        let c = collector(1, [input[i], rules[&input[i..=i+1].iter().collect::<String>().as_str()], input[i+1]].iter().collect::<String>().as_str(), &rules, &mut cache);
        for item in c.keys() {
            *fin.entry(*item).or_insert(0) += c.get(item).unwrap();
        }
    }
    println!("{:?}", fin);
}

fn collector(step: usize, seq: &str, lookup: &HashMap<&str, char>, cache: &mut HashMap<(String, usize), HashMap<char, u128>>) -> HashMap<char, u128> {
    if step == 40 {
        let mut map = HashMap::new();
        for c in 1..seq.len() {
            *map.entry(seq.chars().nth(c).unwrap()).or_insert(0) += 1;
        }
        map
    } else {
        let first_seq = [seq.chars().nth(0).unwrap(), lookup[&seq[0..=1]], seq.chars().nth(1).unwrap()].iter().collect::<String>();
        let mut fin = HashMap::new();
        if cache.contains_key(&(first_seq.clone(), step)) {
            let m = cache.get(&(String::from(first_seq.clone()), step)).unwrap();
            for c in m.keys() {
                *fin.entry(*c).or_insert(0) += m.get(c).unwrap();
            }
        } else {
            let map1 = collector(step + 1, first_seq.as_str(), lookup, cache);
            for c in map1.keys() {
                *fin.entry(*c).or_insert(0) += map1.get(c).unwrap();
            }
            cache.insert((first_seq.clone(), step), map1.clone());
        }

        let second_seq = [seq.chars().nth(1).unwrap(), lookup[&seq[1..=2]], seq.chars().nth(2).unwrap()].iter().collect::<String>();
        if cache.contains_key(&(second_seq.clone(), step)) {
            let m = cache.get(&(String::from(second_seq.clone()), step)).unwrap();
            for c in m.keys() {
                *fin.entry(*c).or_insert(0) += m.get(c).unwrap();
            }
        } else {
            let map2 = collector(step + 1, second_seq.as_str(), lookup, cache);
            for c in map2.keys() {
                *fin.entry(*c).or_insert(0) += map2.get(c).unwrap();
            }
            cache.insert((second_seq.clone(), step), map2.clone());
        }
        fin
    }
}
