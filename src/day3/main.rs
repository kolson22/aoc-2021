fn main() {
    let mut pivot: Vec<Vec<bool>> = vec![];
    let lines: Vec<&str> = include_str!("./day3.test").lines().collect();
    match lines.first() {
        Some(line) => {
            for _ in 0..line.len() {
                pivot.push(vec![]);
            }
        },
        None => panic!("No input file found"),
    };

    let gamma_vec: Vec<bool> = lines.
        iter().
        fold(pivot, |mut p, line| {
            line.chars().zip(&mut p).for_each(|(c,p)| {
                p.push(match c {
                    '1' => true,
                    '0' => false,
                    _ => false,
                });
            });
            p
        }).
        into_iter().
        map(|line| {
            let count = line.iter().filter(|c| **c).count();
            count > line.len() / 2
        }).
        collect();
        println!("Gamma: {:?}", gamma_vec);
}
