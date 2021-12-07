fn main() {
    let input = include_str!("../../input_day7")
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = (0..*input.iter().max().unwrap())
        .into_iter()
        .map(|n| fuel(&input, n))
        .min()
        .unwrap();

    let part2 = (0..*input.iter().max().unwrap())
        .into_iter()
        .map(|n| fuel2(&input, n))
        .min()
        .unwrap();

    println!("Part 1 {}", part1);
    println!("Part 2 {}", part2);
}

fn fuel(input: &Vec<i64>, pos: i64) -> i64 {
    input.iter().map(|&i| (i - pos).abs()).sum()
}

fn fuel2(input: &Vec<i64>, pos: i64) -> i64 {
    input
        .iter()
        .map(|&i| (i - pos).abs())
        .map(|n| n * (n + 1) / 2)
        .sum()
}
