fn parse_input(input: &str) -> Vec<i32> {
    return input.trim().split(',').map(str::parse).map(Result::unwrap).collect::<Vec<i32>>();
}

fn get_fuel_rate(a: i32, items: Vec<i32>) -> i32 {
    let mut output_vec: Vec<i32> = vec![];
    for item in items {
        let mut output = 0;
        let remainder = match item.cmp(&a) {
            std::cmp::Ordering::Less => a - item,
            std::cmp::Ordering::Greater => item - a,
            std::cmp::Ordering::Equal =>  0
        };
        for i in 0..remainder + 1 {
            output = output + i;
        }
        output_vec.push(output);
    }
    return output_vec.iter().sum::<i32>();
}

fn check_number(input: i32, positions: Vec<i32>) -> i32 {
    let result = positions.iter().fold(vec![], |mut output, num| {
        if num > &input {
            output.push(num - input);
        } else {
            output.push(input - num);
        }
        return output;
    });
    return result.iter().sum::<i32>();
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut input_sort: Vec<i32> = input.clone();
    input_sort.sort();
    let mut results = vec![];
    input_sort.iter().for_each(|num| {
        results.push(check_number(*num, input.to_vec()));
    });
    return *results.iter().min().unwrap();
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut results = vec![];
    for x in 0..*input.iter().max().unwrap() {
        results.push(get_fuel_rate(x, input.to_vec()));
    }
    return *results.iter().min().unwrap();
}

fn main() {
    const DATA: &str = include_str!("day5.test");
    let input = parse_input(&DATA);
    println!("Part1 answer: {}", part1(&input));
    println!("Part2 answer: {}", part2(&input));
}

