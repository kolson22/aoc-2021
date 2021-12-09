fn is_larger(a: [&i32; 3], b: [&i32; 3]) -> bool {
    let a: i32 = a.into_iter().sum();
    let b: i32 = b.into_iter().sum();
    return a < b;
}

fn main() {
    let part_1: Vec<i32> = include_str!("./day1.input")
      .split("\n")
      .filter(|item| item.len() > 0)
      .map(|item| item.parse::<i32>())
      .map(|item| item.unwrap())
      .collect();

    let mut count = 0;
    let mut prev = part_1[0];
    for curr in part_1.into_iter().skip(1) {
        if prev < curr {
            count += 1;
        }
        prev = curr;
    }
    println!("The answer is: {}", count);

    let part_2: Vec<i32> = include_str!("./day1_2.input")
      .split("\n")
      .filter(|item| item.len() > 0)
      .map(|item| item.parse::<i32>())
      .map(|item| item.unwrap())
      .collect();

    let mut count = 0;
    let mut prev: [&i32; 3] = [&part_2[0], &part_2[1], &part_2[2]];
    let mut curr: [&i32; 3] = [&part_2[3], &part_2[4], &part_2[5]];
    let mut idx = 3;

    for c in part_2.iter().skip(4) {
        if is_larger(prev, curr) {
            count += 1;
        }
        prev[idx % 3] = curr[(idx - 1) % 3];
        curr[idx % 3] = c;
        idx += 1;
    }

    println!("The answer is: {}", count);
}

