use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
fn main() {
    let input = include_str!("../../input_day9")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = input.len();
    let cols = input[0].len();

    let mut part1 = 0;

    let mut cells = Vec::new();

    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut lower_than = 0;

            if i == 0 || input[i - 1][j] > *cell {
                lower_than += 1;
            }
            if i == rows - 1 || input[i + 1][j] > *cell {
                lower_than += 1;
            }
            if j == 0 || input[i][j - 1] > *cell {
                lower_than += 1;
            }
            if j == cols - 1 || input[i][j + 1] > *cell {
                lower_than += 1;
            }

            if lower_than == 4 {
                part1 += *cell + 1;
                cells.push((i, j));
            }
        }
    }
    println!("Part 1 {}", part1);

    let mut map = input
        .into_iter()
        .map(|v| v.into_iter().map(|c| (c, 0)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut counter = 0;
    for cell in cells {
        counter += 1;
        flood_fill(cell, &mut map, counter);
    }

    let mut sums = HashMap::new();
    for (_, id) in map.clone().into_iter().flatten() {
        if id != 0 {
            sums.entry(id).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let mut heap = sums.values().into_iter().collect::<BinaryHeap<_>>();
    let p2_1 = heap.pop().unwrap();
    let p2_2 = heap.pop().unwrap();
    let p2_3 = heap.pop().unwrap();
    let part2 = p2_1 * p2_2 * p2_3;
    println!("Part 2 {}", part2);
}

fn flood_fill(pos: (usize, usize), map: &mut Vec<Vec<(u32, u32)>>, id_counter: u32) {
    let rows = map.len();
    let cols = map[0].len();

    if pos.0 >= rows || pos.1 >= cols {
        return;
    }

    if map[pos.0][pos.1].0 == 9 || map[pos.0][pos.1].1 != 0 {
        return;
    }
    map[pos.0][pos.1].1 = id_counter;
    if pos.0 > 0 {
        flood_fill((pos.0 - 1, pos.1), map, id_counter);
    }
    flood_fill((pos.0 + 1, pos.1), map, id_counter);
    if pos.1 > 0 {
        flood_fill((pos.0, pos.1 - 1), map, id_counter);
    }
    flood_fill((pos.0, pos.1 + 1), map, id_counter);
}
