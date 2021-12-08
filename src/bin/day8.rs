extern crate itertools;
extern crate rayon;
use itertools::iproduct;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let part1 = include_str!("../../input_day8")
        .lines()
        .map(|l| {
            l.split("|")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|s| s.len())
                .filter(|&l| (l == 2 || l == 3 || l == 4 || l == 7))
                .count()
        })
        .sum::<usize>();

    println!("Part 1 {}", part1);

    let input = include_str!("../../input_day8")
        .lines()
        .map(|l| {
            let mut parts = l.split("|");
            let patterns = parts.next().unwrap();
            let output = parts.next().unwrap();
            (parse_and_sort(patterns), parse_and_sort(output))
        })
        .collect::<Vec<_>>();

    let chars = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);

    let mut mappings = HashMap::new();
    mappings.insert("abcefg".chars().collect(), 0);
    mappings.insert("cf".chars().collect(), 1);
    mappings.insert("acdeg".chars().collect(), 2);
    mappings.insert("acdfg".chars().collect(), 3);
    mappings.insert("bcdf".chars().collect(), 4);
    mappings.insert("abdfg".chars().collect(), 5);
    mappings.insert("abdefg".chars().collect(), 6);
    mappings.insert("acf".chars().collect(), 7);
    mappings.insert("abcdefg".chars().collect(), 8);
    mappings.insert("abcdfg".chars().collect(), 9);

    let mut part2 = 0;
    for (patterns, outputs) in input {
        let solver = Solver {
            chars: &chars,
            patterns: &patterns,
            mappings: &mappings,
        };
        let solution = solver.solve(&HashMap::new()).unwrap();
        let mut res = String::from("");
        for output in outputs {
            let mut converted = output
                .into_iter()
                .map(|c| *solution.get(&c).unwrap())
                .collect::<Vec<_>>();
            converted.sort();
            for (k, v) in &mappings {
                if *k == converted {
                    res = format!("{}{}", res, v);
                }
            }
        }
        part2 += res.parse::<i32>().unwrap();
        println!("Res {:?}", res);
    }

    println!("Part2 {:?}", part2);
}

fn parse_and_sort(patterns: &str) -> Vec<Vec<char>> {
    patterns
        .split_whitespace()
        .map(|token| {
            let mut vector = token.chars().collect::<Vec<_>>();
            vector.sort();
            vector.into_iter().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

enum Result {
    Partial,
    Full,
    Fail,
}

struct Solver<'a> {
    chars: &'a HashSet<char>,
    patterns: &'a Vec<Vec<char>>,
    mappings: &'a HashMap<Vec<char>, i32>,
}

impl<'a> Solver<'a> {
    fn solve(&self, solution: &HashMap<char, char>) -> Option<HashMap<char, char>> {
        let from_chars_used = solution.keys().cloned().collect();
        let from_chars_left = self
            .chars
            .difference(&from_chars_used)
            .cloned()
            .collect::<Vec<_>>();

        let to_chars_used = solution.values().cloned().collect();
        let to_chars_left = self
            .chars
            .difference(&to_chars_used)
            .cloned()
            .collect::<Vec<_>>();

        let left = iproduct!(from_chars_left.iter(), to_chars_left.iter()).collect::<Vec<_>>();
        left.par_iter()
            .map(|&(a, i)| {
                let mut new_solution = solution.clone();
                new_solution.insert(a.clone(), i.clone());

                match self.check(&new_solution) {
                    Result::Full => Some(new_solution),
                    Result::Partial => self.solve(&new_solution),
                    Result::Fail => None,
                }
            })
            .find_any(Option::is_some)
            .map(Option::unwrap)
    }

    fn check(&self, solution: &HashMap<char, char>) -> Result {
        for pattern in self.patterns {
            let res = self
                .mappings
                .keys()
                .into_iter()
                .filter(|&mapping| mapping.len() == pattern.len())
                .any(|mapping| {
                    let mut mapped = pattern
                        .into_iter()
                        .map(|c| *solution.get(c).unwrap_or(&'_'))
                        .collect::<Vec<_>>();
                    mapped.sort();

                    mapped.contains(&'_') || mapped == *mapping
                });
            if !res {
                return Result::Fail;
            }
        }

        if solution.len() == self.chars.len() {
            return Result::Full;
        } else {
            return Result::Partial;
        }
    }
}
