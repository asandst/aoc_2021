fn main() {
    let input = include_str!("../../input_day11")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap(), false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1_map = input.clone();
    let mut part1 = 0;

    for _ in 0..100 {
        simulate(&mut part1_map, &mut part1);
    }
    println!("Part 1 {}", part1);

    let mut part2_map = input.clone();
    let mut part2 = 0;

    for steps in 1..10000 {
        let mut flashes = 0;
        simulate(&mut part2_map, &mut flashes);

        if flashes == part2_map.len() * part2_map[0].len() {
            part2 = steps;
            break;
        }
    }
    println!("Part 2 {}", part2);
}

fn simulate(map: &mut Vec<Vec<(u32, bool)>>, flashes: &mut usize) {
    map.iter_mut().flatten().for_each(|(_, f)| *f = false);
    map.iter_mut().flatten().for_each(|(v, _)| *v += 1);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            flash((i, j), map, flashes);
        }
    }
}

fn flash(pos: (usize, usize), map: &mut Vec<Vec<(u32, bool)>>, flashes: &mut usize) {
    if map[pos.0][pos.1].1 == false && map[pos.0][pos.1].0 > 9 {
        map[pos.0][pos.1] = (0, true);
        *flashes += 1;

        for i in pos.0.saturating_sub(1)..=(pos.0 + 1).clamp(0, map.len() - 1) {
            for j in pos.1.saturating_sub(1)..=(pos.1 + 1).clamp(0, map[0].len() - 1) {
                if map[i][j].1 == false {
                    map[i][j].0 += 1;
                    flash((i, j), map, flashes);
                }
            }
        }
    }
}
