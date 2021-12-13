use std::collections::HashSet;

fn main() {
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    let mut folds = Vec::new();
    let lines = include_str!("../../input_day13").lines();
    let mut parse_folds = false;
    for line in lines {
        if line.is_empty() {
            parse_folds = true;
        } else if !parse_folds {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            map.insert((x, y));
        } else {
            let l = line.replace("fold along ", "");
            let value = l.replace("x=", "").replace("y=", "").parse().unwrap();
            if l.starts_with("x=") {
                folds.push((value, usize::MAX))
            } else {
                folds.push((usize::MAX, value))
            }
        }
    }

    let x_max = map.iter().map(|&(x, _)| x).max().unwrap();
    let part1 = map
        .clone()
        .into_iter()
        .map(&|(x, y)| (if x > 655 { x_max - x } else { x }, y))
        .collect::<HashSet<_>>();

    println!("Part 1 {:?}", part1.len());

    let mut part2_set = map.clone();
    let mut y_max = map.iter().map(|&(_, y)| y).max().unwrap();
    let mut x_max = map.iter().map(|&(x, _)| x).max().unwrap();

    for (x_fold, y_fold) in folds {
        part2_set = part2_set
            .into_iter()
            .map(|(x, y)| {
                (
                    if x > x_fold { x_max - x } else { x },
                    if y > y_fold { y_max - y } else { y },
                )
            })
            .collect::<HashSet<_>>();
        y_max = part2_set.iter().map(|&(_, y)| y).max().unwrap();
        x_max = part2_set.iter().map(|&(x, _)| x).max().unwrap();
    }

    let mut part2 = vec![vec!['-'; x_max + 1]; y_max + 1];
    part2_set.into_iter().for_each(|(x, y)| part2[y][x] = '#');

    for row in part2 {
        for c in row {
            print!("{}", c)
        }
        println!("");
    }
}
