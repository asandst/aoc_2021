fn main() {
    let input = include_str!("../../input_day16")
        .chars()
        .map(|c| format!("{:004b}", c.to_digit(16).unwrap()))
        .collect::<String>();

    let mut version_sum = 0;
    let mut values = Vec::new();
    parse_packet(&mut input.to_string(), &mut version_sum, &mut values, false);
    println!("Part1 {}", version_sum);
    println!("Part2 {}", values.first().unwrap());
}

fn parse_packet(input: &mut String, v_sum: &mut u32, values: &mut Vec<i64>, once: bool){
    while input.len() > 3 {
        let version = input.drain(..3).collect::<String>();
        let p_type = input.drain(..3).collect::<String>();
        let p_type = isize::from_str_radix(&p_type, 2).unwrap();
        *v_sum += u32::from_str_radix(&version, 2).unwrap();

        let value = match p_type {
            4 => value_packet(input),
            _ => op_packet(input, p_type, v_sum),
        };
        values.push(value);

        if once {
            break;
        }
    }
}

fn op_packet(input: &mut String, p_type: isize, version_sum: &mut u32) -> i64 {
    let length_type = input.drain(..1).collect::<String>();
    let mut values = Vec::new();
    match length_type.as_str() {
        "0" => op_type_0(input, version_sum, &mut values),
        _ => op_type_1(input, version_sum, &mut values),
    }
    match p_type {
        0 => values.iter().sum::<i64>(),
        1 => values.iter().product::<i64>(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => (values.first().unwrap() > values.last().unwrap()) as i64,
        6 => (values.first().unwrap() < values.last().unwrap()) as i64,
        7 => (values.first().unwrap() == values.last().unwrap()) as i64,
        _ => panic!(),
    }
}

fn op_type_1(input: &mut String, version_sum: &mut u32, values: &mut Vec<i64>) {
    let number_of_sub_packets = input.drain(..11).collect::<String>();
    let number_of_sub_packets = isize::from_str_radix(&number_of_sub_packets, 2).unwrap();
    for _ in 0..number_of_sub_packets {
        parse_packet(input, version_sum, values, true);
    }
}

fn op_type_0(input: &mut String, version_sum: &mut u32, values: &mut Vec<i64>) {
    let length = input.drain(..15).collect::<String>();
    let length = usize::from_str_radix(&length, 2).unwrap();
    let mut data = input.drain(..length).collect();
    parse_packet(&mut data, version_sum, values, false);
}

fn value_packet(input: &mut String) -> i64 {
    let mut prefix = input.drain(..1).collect::<String>();
    let mut value = "".to_string();

    while prefix == "1" {
        let value_part = input.drain(..4).collect::<String>();
        value = format!("{}{}", value, value_part);
        prefix = input.drain(..1).collect();
    }
    let value_part = input.drain(..4).collect::<String>();
    value = format!("{}{}", value, value_part);
    i64::from_str_radix(&value, 2).unwrap()
}
