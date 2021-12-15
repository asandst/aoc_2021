fn main() {
    let input = include_str!("../../input_day15")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part1 {:?}", dijsktra(&input));

    let mut input2 = vec![vec![0; input[0].len() * 5]; input.len() * 5];
    for i in 0..5 {
        for j in 0..5 {
            for (x, row) in input.iter().enumerate() {
                for (y, &v) in row.iter().enumerate() {
                    let mut v = v;
                    if j > 0 {
                        v = input2[x + i * input.len()][y + (j - 1) * input[0].len()] + 1;
                        if v > 9 {
                            v = 1;
                        }
                    } else if i > 0 {
                        v = input2[x + (i - 1) * input.len()][y + j * input[0].len()] + 1;
                        if v > 9 {
                            v = 1;
                        }
                    }

                    input2[x + i * input.len()][y + j * input[0].len()] = v;
                }
            }
        }
    }
    println!("Part2 {:?}", dijsktra(&input2));
}

fn dijsktra(input: &Vec<Vec<u32>>) -> u32 {
    let mut distances = vec![vec![u32::MAX; input[0].len()]; input.len()];
    distances[0][0] = 0;
    let mut heap: Vec<QNode> = Vec::new();
    for (i, row) in distances.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            heap.push(QNode { x: i, y: j });
        }
    }
    heap.sort_by(|a, b| distances[b.x][b.y].cmp(&distances[a.x][a.y]));
    while let Some(v) = heap.pop() {
        if v.x > 0 {
            let l = distances[v.x][v.y] + input[v.x - 1][v.y];
            if l < distances[v.x - 1][v.y] {
                distances[v.x - 1][v.y] = l;
            }
        }

        if v.x < input.len() - 1 {
            let l = distances[v.x][v.y] + input[v.x + 1][v.y];
            if l < distances[v.x + 1][v.y] {
                distances[v.x + 1][v.y] = l;
            }
        }

        if v.y > 0 {
            let l = distances[v.x][v.y] + input[v.x][v.y - 1];
            if l < distances[v.x][v.y - 1] {
                distances[v.x][v.y - 1] = l;
            }
        }

        if v.y < input[0].len() - 1 {
            let l = distances[v.x][v.y] + input[v.x][v.y + 1];
            if l < distances[v.x][v.y + 1] {
                distances[v.x][v.y + 1] = l;
            }
        }
        heap.sort_by(|a, b| distances[b.x][b.y].cmp(&distances[a.x][a.y]));
    }

    distances[input.len() - 1][input[0].len() - 1]
}
#[derive(Debug)]
struct QNode {
    x: usize,
    y: usize,
}
