use std::{collections::HashSet, process::exit};

fn main() {
    let mut input = include_str!("../../input_day4").lines();

    let numbers = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = Vec::new();

    while input.next().is_some() {
        let mut board = Vec::new();
        for _ in 0..5 {
            let row = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| (n.parse().unwrap(), false))
                .collect::<Vec<_>>();
            board.push(row);
        }
        boards.push(board);
    }

    let mut part1 = None;
    let mut part2 = HashSet::new();

    for number in numbers {
        for board in &mut boards {
            for row in board {
                for mut pos in row {
                    if pos.0 == number {
                        pos.1 = true;
                    }
                }
            }
        }

        for (i, board) in boards.iter().enumerate() {
            if check_win(&board) {
                part2.insert(i);
                if part1 == None {
                    part1 = Some(score(&board, number));
                    println!("Part 1 {}", part1.unwrap());
                }

                if part2.len() == boards.len() {
                    let p2 = score(&board, number);
                    println!("Part 2 {}", p2);
                    exit(0);
                }
            }
        }
    }
}

fn check_win(board: &Vec<Vec<(u32, bool)>>) -> bool {
    for i in 0..5 {
        let mut matched1 = true;
        let mut matched2 = true;
        for j in 0..5 {
            matched1 = matched1 && board[i][j].1;
            matched2 = matched2 && board[j][i].1;
        }
        if matched1 || matched2 {
            return true;
        }
    }

    false
}

fn score(board: &Vec<Vec<(u32, bool)>>, number: u32) -> u32 {
    board
        .into_iter()
        .flatten()
        .filter(|p| !p.1)
        .map(|p| p.0)
        .sum::<u32>()
        * number
}
