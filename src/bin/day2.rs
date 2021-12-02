fn main() {
    let input = include_str!("../../input_day2")
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut parts| {
            let text = parts.next().unwrap();
            let length: i32 = parts.next().unwrap().parse().unwrap();

            match text {
                "forward" => (length, 0),
                "up" => (0, -length),
                "down" => (0, length),
                _ => panic!(),
            }
        })
        .collect::<Vec<(i32, i32)>>();

    let part1 = input
        .clone()
        .into_iter()
        .reduce(|(a_pos, a_depth), (pos, depth)| (a_pos + pos, a_depth + depth))
        .map(|(pos, depth)| pos * depth)
        .unwrap();
    println!("Part 1 {}", part1);

    let accum = input
        .into_iter()
        .fold((0, 0, 0), |(a_pos, a_aim, a_depth), (pos, aim)| {
            (a_pos + pos, a_aim + aim, a_depth + pos * a_aim)
        });
    let part2 = accum.0 * accum.2;
    println!("Part 2 {}", part2);
}
