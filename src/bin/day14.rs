use std::collections::{HashMap};

fn main() {
    
    let lines = include_str!("../../input_day14").lines();
    let mut parse_mappings = false;

    let mut input_polymer ="".to_string();
    let mut mappings = HashMap::new(); 

    for line in lines {
        if line.is_empty() {
            parse_mappings = true;
        } else if !parse_mappings {
            input_polymer = line.to_string();
        } else {
            let mut parts = line.split("->");
            let from = parts.next().unwrap().trim();
            let to = parts.next().unwrap().trim().chars().next().unwrap();
            mappings.insert(from, to);
        }
    }

    let part1 = expand_polymer(input_polymer.clone(), &mappings, 10);

    let mut res = HashMap::new();
    for c in part1.chars(){
        *res.entry(c).or_insert(0) += 1;
    }
    let min = res.values().min().unwrap();
    let max = res.values().max().unwrap();
    println!("Part 1 {:?}", max-min);

    let part2 = expand_polymer(input_polymer.clone(), &mappings, 20);
    let mut part2_map = HashMap::new();
    for key in mappings.keys() {
        let expanded = expand_polymer(key.to_string(), &mappings, 20);
        let mut res = HashMap::new();
        for (i, c) in expanded.chars().enumerate(){
            if i < expanded.len()-1{
                *res.entry(c).or_insert(0) += 1;
            }
            
        }
        part2_map.insert(*key, res);
    }

    let mut i = 0;
    let mut sums = HashMap::new();
    let end_c;
    loop{
        let s = &part2[i..(i+2)];
        part2_map[s].iter().for_each(|(k, v)| *sums.entry(k).or_insert(0u64) += v);
        i += 1;

        if i >= part2.len()-1{
            let end = &part2[i..(i+1)];
            end_c = end.clone().chars().next().unwrap();
            *sums.entry(&end_c).or_insert(0u64) += 1;
            break;
        }
    }

    let min = sums.values().min().unwrap();
    let max = sums.values().max().unwrap();
    println!("Part 2 {:?}", max-min);
    
}

fn expand_polymer(mut polymer: String, mappings: &HashMap<&str, char>, steps: u32) -> String{

    for _ in 0..steps{
        let mut new_str = "".to_string();
        let mut i = 0;
        loop{
            let s = &polymer[i..(i+2)];

            new_str.push(polymer[i..i+1].chars().next().unwrap());
        
            if mappings.contains_key(s){
                new_str.push(mappings[s]);
            }
            i += 1;

            if i >= polymer.len()-1{
                new_str.push(polymer[i..i+1].chars().next().unwrap());
                break;
            }
        }
        polymer = new_str;
    }
    polymer
}