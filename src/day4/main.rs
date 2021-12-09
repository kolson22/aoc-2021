#[derive(Debug, Copy, Clone)]
enum BoardSlot {
    Unmarked(i32),
    Marked(i32),
}

#[derive(Debug)]
struct Board {
    board: [[BoardSlot; 5]; 5],
    marked: [[BoardSlot; 5]; 5],
}

pub fn parse_input(input: &str) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<i32> = lines[0]
        .split(',')
        .map(str::parse).map(Result::unwrap)
        .collect();
    return numbers;
}

pub fn parse_board(input: &str) {
    let boards: Vec<String> = input.trim().lines().skip(1).map(str::parse).map(Result::unwrap).collect();
    let board1: Vec<Vec<u32>> = boards.into_iter().take(6).skip(1).map(|e| e.split_whitespace()).flatten().map(|item| item.parse::<u32>().unwrap()).collect();
    println!("{:?}", board1)
}

pub fn part_1(input: &str) -> Vec<i32> {
    let numbers = parse_input(input);
    for i in 0..numbers.len() {
        // println!("numbers: {}", numbers[i]);
    }
    return parse_input(input);
}

fn main() {
    const DATA: &str = include_str!("./day4.test");
    let result = part_1(&DATA);
    parse_board(&DATA);
    println!("result is: {:?}", result);
}
