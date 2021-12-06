fn main() {
    let input = include_str!("../../input_day6")
        .split(",")
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut state = vec!(0u64;9);
    for i in input {
        state[i as usize] += 1;
    }

    let part1 = sim(80, state.clone());
    println!("Part 1 {}", part1);
    let part2 = sim(256, state);
    println!("Part 2 {}", part2);
}

fn sim(days: u32, mut state: Vec<u64>) -> u64 {
    for _ in 0..days{
        let spawns = state[0];
        for index in 0..=7 {
            state[index] = state[index+1];
        }
        state[8] = spawns;
        state[6] += spawns;
    }
    state.into_iter().sum()
}

