fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.iter().fold(0, |mut acc, line| {
        let (_, output) = line.split_once(" | ").unwrap();
        let count = output.split_whitespace().fold(0, |mut total, word| {
            if [2, 3, 4, 7].contains(&word.len()) {
                total += 1;
            }
            return total;
        });
        acc += count;
        return acc;
    })
}

fn sort_and_join(s: &str) -> String {
    let mut output: Vec<char> = s.chars().collect();
    output.sort_by(|a, b| a.cmp(b));
    return output.iter().collect::<String>();
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let output = vec![0u32; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let (inp, out) = line.split_once(" | ").unwrap();
        // inp.iter().fold(init, f)
    }
    // output[1] = input.split_once(" | ").unwrap().1.split_whitespace().filter(|x| x.len() == 2).collect().sum();
    return 5;
}

fn test_fn() {
    let v: Vec<_> = (0..).zip("ABCDE".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
    println!("{:?}", v);
}

fn main() {
    const DATA: &str = include_str!("day8.test");
    println!("answer part1: {}", part1(&DATA));
    let test = &DATA.split_whitespace().collect::<Vec<&str>>()[0];
    println!("before: {}, after: {}", test, sort_and_join(test));
    println!("answer part2: {}", part2(&DATA));
    test_fn();
}
