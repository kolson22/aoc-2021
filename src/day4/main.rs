use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum BoardSlot {
    Unmarked(i32),
    Marked(i32),
}

#[derive(Debug)]
struct Board {
    board: [[BoardSlot; 5]; 5],
}

pub fn parse_input(input: &str) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<i32> = lines[0]
        .split(',')
        .map(str::parse).map(Result::unwrap)
        .collect();
    return numbers;
}

pub fn parse_boards(input: &str) {
    let mut lines: Vec<&str> = input.lines().collect();
    let boards: Vec<&str> = lines[1..]
        .chunks(6)
        .into_iter()
        .skip(1)
        .map(|mut lines| lines.join("\n")[1..].parse())
        .filter_map(Result::ok)
        .collect();
        println!("{:?}", boards);
}

pub fn part_1(input: &str) -> anyhow::Result<Vec<i32>> {
    return Ok(parse_numbers(input));
}

fn main() {
    const DATA: &str = include_str!("./day4.test");
    let result = part_1(&DATA);
    println!("result is: {:?}", result.unwrap());
}
