fn parse_lines(data: &str) -> Vec<u32> {
    data.trim().lines().map(str::parse).map(Result::unwrap).collect()
}

fn part_1(data: &str) -> u32 {
    let lines = parse_lines(data);
    let mut count = 0;
    for i in 1..lines.len() {
        if lines[i] > lines[i - 1] {
            count += 1;
        }
    }
    return count;
}

fn part_2(data: &str) -> u32 {
    let lines = parse_lines(data);
    let mut count = 0;
    for i in 0..(lines.len() - 3) {
        let first: u32 = [lines[i],lines[i + 1],lines[i + 2]].iter().sum();
        let second: u32 = [lines[i + 1],lines[i + 2],lines[i + 3]].iter().sum();
        if second > first {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let data: &str = include_str!("day1.input");
    println!("Part 1 answer is: {}", part_1(&data));
    println!("Part 2 answer is: {}", part_2(&data));
}
