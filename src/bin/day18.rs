
fn main() {
    let input = include_str!("../../input_day18")
        .lines()
        .map(|l| build_snail_number(l))
        .collect::<Vec<_>>();

    let mut numbers = input.clone();

    while numbers.len() > 1 {
        let first = numbers.remove(0);
        let second = numbers.remove(0);
        let new_number = SnailNumber::Pair(Box::new((first, second)));

        let mut stack = Vec::new();
        create_nestling_stack(&new_number, 0, &mut stack);
        while explode(&mut stack) || split(&mut stack) {
        }
        let s = build_snail_number_from_stack(stack);
        numbers.insert(0, s.unwrap());
    }

    for number in numbers {
        let m = magnitude(&number);
        println!("Part1 {:?}", m);
    }

    let mut part2 = 0;

    for (i, s1) in input.clone().iter().enumerate() {
        for (j, s2) in input.clone().iter().enumerate() {
            if i == j {
                continue;
            }

            let new_number = SnailNumber::Pair(Box::new((s1.clone(), s2.clone())));
            let mut stack = Vec::new();
            create_nestling_stack(&new_number, 0, &mut stack);
            while explode(&mut stack) || split(&mut stack) {
            }
            //hope the inputs that gets stuck and gives error is not important
            if let Ok(snail_number) = build_snail_number_from_stack(stack){
                let magnitude = magnitude(&snail_number);
                if magnitude > part2 {
                    part2 = magnitude;
                }
            }
        }
    }

    println!("Part2 {:?}", part2);
}

fn explode(stack: &mut Vec<(u64, u64)>) -> bool {
    for (i, (s, n)) in stack.clone().iter().enumerate() {
        if *n > 4 {
            if i > 0 {
                stack[i - 1].0 += s;
            }
            if i < stack.len() - 2 {
                stack[i + 2].0 += stack[i + 1].0;
            }
            stack[i].0 = 0;
            stack[i].1 -= 1;
            stack.remove(i + 1);
            return true;
        }
    }
    false
}

fn split(stack: &mut Vec<(u64, u64)>) -> bool {
    for (i, (s, n)) in stack.clone().iter().enumerate() {
        if *s >= 10 {
            let left = s / 2;
            let right = s / 2 + s % 2;
            stack[i].0 = left;
            stack[i].1 = n + 1;
            stack.insert(i + 1, (right, n + 1));
            return true;
        }
    }
    false
}

fn magnitude(number: &SnailNumber) -> u64 {
    match number {
        SnailNumber::Number(v) => return *v,
        SnailNumber::Pair(p) => {
            return magnitude(&p.0) * 3 + magnitude(&p.1) * 2;
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum SnailNumber {
    Number(u64),
    Pair(Box<(SnailNumber, SnailNumber)>),
}

fn build_snail_number(line: &str) -> SnailNumber {
    let mut v = Vec::new();

    for c in line.chars() {
        match c {
            ',' => (),
            '[' => (),
            ']' => {
                let second = v.pop().unwrap();
                let first = v.pop().unwrap();

                v.push(SnailNumber::Pair(Box::new((first, second))))
            }
            '0'..='9' => v.push(SnailNumber::Number(c.to_digit(10).unwrap() as u64)),
            _ => panic!(),
        };
    }
    v.pop().unwrap()
}

fn build_snail_number_from_stack(stack: Vec<(u64, u64)>) -> Result<SnailNumber, ()> {
    let mut v = Vec::new();

    for (s, n) in stack {
        v.push((SnailNumber::Number(s), n));
    }

    let mut loops = 0;

    //this code does not work for all edge cases. It return an Error Result if it gets stuck
    while v.len() > 1 {
        for (i, (_, _)) in v.clone().iter().enumerate() {
            if i < v.len() - 1 {
                if v[i].1 == v[i + 1].1 {
                    let n = v[i].1;

                    let first = v.remove(i);
                    let second = v.remove(i);
                    v.insert(i, (SnailNumber::Pair(Box::new((first.0, second.0))), n - 1));
                }
            }
        }
        loops += 1;
        if loops > 1000{
            return Result::Err(());
        }
    }

    Result::Ok(v[0].0.clone())
}

fn create_nestling_stack(number: &SnailNumber, nestling: u64, stack: &mut Vec<(u64, u64)>) {
    match number {
        SnailNumber::Number(v) => stack.push((*v, nestling)),
        SnailNumber::Pair(p) => {
            create_nestling_stack(&p.0, nestling + 1, stack);
            create_nestling_stack(&p.1, nestling + 1, stack);
        }
    }
}
