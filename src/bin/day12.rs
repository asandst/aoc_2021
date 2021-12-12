use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input_day12")
        .lines()
        .map(|line| {
            let mut parts = line.split("-");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect::<Vec<_>>();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (start, end) in input.clone() {
        map.entry(start).or_insert_with(Vec::new).push(end);
        map.entry(end).or_insert_with(Vec::new).push(start);
    }

    let solver = Solver {
        map: &map,
        single_small_twice: false,
    };
    let solution = solver.solve(&vec!["start"]);
    println!("Part 1 {:?}", solution.len());

    let solver = Solver {
        map: &map,
        single_small_twice: true,
    };
    let solution = solver.solve(&vec!["start"]);
    println!("Part 2 {:?}", solution.len());
}

enum Result {
    Partial,
    Full,
}

struct Solver<'a> {
    map: &'a HashMap<&'a str, Vec<&'a str>>,
    single_small_twice: bool,
}

impl<'a> Solver<'a> {
    fn solve(&self, solution: &Vec<&'a str>) -> Vec<Vec<&str>> {
        let used_small = solution
            .iter()
            .filter(|&s| s.to_ascii_lowercase() == *s)
            .collect::<Vec<_>>();
        let unique_small_used = used_small.iter().collect::<HashSet<_>>().len();

        let allow_small_twice = self.single_small_twice && unique_small_used == used_small.len();
        self.map[solution.last().unwrap()]
            .iter()
            .filter(|&&s| s != "start")
            .filter(|s| allow_small_twice || !used_small.contains(&s))
            .map(|s| {
                let mut new_solution = solution.clone();
                new_solution.push(s.clone());

                match self.check(&new_solution) {
                    Result::Full => vec![new_solution],
                    Result::Partial => self.solve(&new_solution),
                }
            })
            .flatten()
            .collect()
    }

    fn check(&self, solution: &Vec<&str>) -> Result {
        let last = solution.last();
        if last.is_some() && last.unwrap() == &"end" {
            return Result::Full;
        } else {
            return Result::Partial;
        }
    }
}
