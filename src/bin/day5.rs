use regex::Regex;
fn main() {
    let regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) \-> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    let input = include_str!("../../input_day5")
        .lines()
        .into_iter()
        .map(|line| regex.captures(line).unwrap())
        .map(|c| {
            (
                create_range(c["x1"].parse().unwrap(), c["x2"].parse().unwrap()),
                create_range(c["y1"].parse().unwrap(), c["y2"].parse().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut map1 = vec![vec![0usize; 1000]; 1000];
    let mut map2 = vec![vec![0usize; 1000]; 1000];

    for (x_range, y_range) in input {
        let is_diag = x_range.len() == y_range.len();
        for (i, x) in x_range.into_iter().enumerate() {
            for (j, y) in y_range.clone().into_iter().enumerate() {
                if !is_diag {
                    map1[y as usize][x as usize] += 1;
                    map2[y as usize][x as usize] += 1;
                } else if i == j {
                    map2[y as usize][x as usize] += 1;
                }
            }
        }
    }

    let part1 = map1.into_iter().flatten().filter(|&v| v > 1).count();
    println!("Part 1 {}", part1);
    let part2 = map2.into_iter().flatten().filter(|&v| v > 1).count();
    println!("Part 2 {}", part2);
}

fn create_range(start: i32, end: i32) -> Vec<i32> {
    let mut i = start;
    let dir = if start > end { -1 } else { 1 };
    let mut res = Vec::new();
    while i != end {
        res.push(i);
        i += dir;
    }
    res.push(end);
    res
}
