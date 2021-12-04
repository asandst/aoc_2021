fn main() {
    let input = include_str!("../../input_day3")
        .lines()
        .map(|line| isize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..12 {
        let ones = ones_at_bit_index(&input, i);
        if ones > (input.len() / 2) as i32 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    let part1 = epsilon * gamma;
    println!("Part 1 {}", part1);

    let mut ogr_cand = input.clone();
    let mut csr_cand = input.clone();
    for i in (0..12).rev() {
        let ones_ogr = ones_at_bit_index(&ogr_cand, i);
        let ones_csr = ones_at_bit_index(&csr_cand, i);

        let ogr = ones_ogr as f64 >= ogr_cand.len() as f64 / 2.0;
        let csr = (ones_csr as f64) < csr_cand.len() as f64 / 2.0;

        if ogr_cand.len() > 1 {
            ogr_cand = find_matching(ogr_cand, i, ogr as isize);
        }
        if csr_cand.len() > 1 {
            csr_cand = find_matching(csr_cand, i, csr as isize);
        }
    }

    let part2 = ogr_cand[0] * csr_cand[0];
    println!("Part 2 {}", part2);
}

fn find_matching(candidates: Vec<isize>, i: i32, value: isize) -> Vec<isize> {
    candidates
        .into_iter()
        .filter(|n| ((n >> i) & 1) == value)
        .collect()
}

fn ones_at_bit_index(numbers: &Vec<isize>, i: i32) -> i32 {
    numbers.iter().filter(|&&n| (n >> i) % 2 == 1).count() as i32
}
