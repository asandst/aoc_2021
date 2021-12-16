fn main() {
    let input = include_str!("../../input_day16")
        .chars()
        .map(|c| format!("{:004b}", c.to_digit(16).unwrap()))
        .collect::<String>();

    let mut version_sum = 0;
    let mut values = Vec::new();
    parse_packet(&input, &mut version_sum, &mut values, false);
    println!("Part1 {}", version_sum);
    println!("Part2 {}", values.first().unwrap());
}

fn parse_packet<'a>(
    mut input: &'a str,
    v_sum: &mut u32,
    values: &mut Vec<i64>,
    once: bool,
) -> &'a str {
    while input.len() > 3 {
        let (version, inner_input) = input.split_at(3);
        let (p_type, inner_input) = inner_input.split_at(3);
        let p_type = isize::from_str_radix(p_type, 2).unwrap();
        *v_sum += isize::from_str_radix(version, 2).unwrap() as u32;

        let (inner_input, value) = match p_type {
            4 => value_packet(inner_input),
            _ => op_packet(inner_input, p_type, v_sum),
        };
        input = inner_input;
        values.push(value);

        if once {
            break;
        }
    }
    input
}

fn op_packet<'a>(input: &'a str, p_type: isize, version_sum: &mut u32) -> (&'a str, i64) {
    let (length_type, input) = input.split_at(1);
    let mut values = Vec::new();
    (
        match length_type {
            "0" => op_type_0(input, version_sum, &mut values),
            _ => op_type_1(input, version_sum, &mut values),
        },
        match p_type {
            0 => values.iter().sum::<i64>(),
            1 => values.iter().product::<i64>(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => (values.first().unwrap() > values.last().unwrap()) as i64,
            6 => (values.first().unwrap() < values.last().unwrap()) as i64,
            7 => (values.first().unwrap() == values.last().unwrap()) as i64,
            _ => panic!(),
        },
    )
}

fn op_type_1<'a>(input: &'a str, version_sum: &mut u32, values: &mut Vec<i64>) -> &'a str {
    let (number_of_sub_packets, input) = input.split_at(11);
    let number_of_sub_packets = isize::from_str_radix(number_of_sub_packets, 2).unwrap();
    let mut input = input;
    for _ in 0..number_of_sub_packets {
        input = parse_packet(input, version_sum, values, true);
    }
    input
}

fn op_type_0<'a>(input: &'a str, version_sum: &mut u32, values: &mut Vec<i64>) -> &'a str {
    let (length, input) = input.split_at(15);
    let length = isize::from_str_radix(length, 2).unwrap();
    let (data, input) = input.split_at(length as usize);
    parse_packet(data, version_sum, values, false);
    input
}

fn value_packet(input: &str) -> (&str, i64) {
    let (mut prefix, mut input) = input.split_at(1);
    let mut value = "".to_string();

    while prefix == "1" {
        let (value_part, inner_input) = input.split_at(4);
        value = format!("{}{}", value, value_part);
        let (inner_prefix, inner_input) = inner_input.split_at(1);
        prefix = inner_prefix;
        input = inner_input;
    }
    let (value_part, input) = input.split_at(4);
    value = format!("{}{}", value, value_part);
    let value = isize::from_str_radix(&value, 2).unwrap();

    (input, value as i64)
}
