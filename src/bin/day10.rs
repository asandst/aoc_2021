use std::collections::{BinaryHeap, HashMap, VecDeque};

fn main() {
    let input = include_str!("../../input_day10")
        .lines()
        .collect::<Vec<_>>();

    let mut part1 = 0;

    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('<', '>');

    let mut part2_scores = Vec::new();

    for line in input {
        let mut stack = VecDeque::new();
        let mut corrupted = 0;
        for c in line.chars() {
            corrupted = match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_front(c);
                    0
                }
                ')' | ']' | '}' | '>' => {
                    let front = stack.pop_front().unwrap();
                    let mapped = map[&front];
                    if mapped != c {
                        match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!(),
                        }
                    } else {
                        0
                    }
                }
                _ => panic!(),
            };

            if corrupted != 0 {
                part1 += corrupted;

                break;
            }
        }

        if corrupted == 0 {
            let mut score: u64 = 0;
            for top in stack {
                let mapped = map[&top];
                let s = match mapped {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                };

                score = score * 5 + s;
            }
            part2_scores.push(score);
        }
    }
    part2_scores.sort();

    let part2 = part2_scores[part2_scores.len() / 2];

    println!("Part 1 {}", part1);
    println!("Part 2 {}", part2);
}
