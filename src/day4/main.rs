#[derive(Debug, Copy, Clone)]
enum BoardSlot {
    Unmarked(i32),
    Marked(i32),
}

#[derive(Debug, Copy, Clone)]
struct Board {
    board: [[BoardSlot; 5]; 5]
}

impl Board {
    fn mark(&mut self, value: i32) -> bool {
        for col in 0..5 {
            for row in 0..5 {
                if self.board[col][row].is_equal(value) {
                    self.board[col][row] = BoardSlot::Marked(value);
                    return self.check_for_win(col, row);
                }
            }
        }
        return false;
    }

    fn check_for_win(&self, col: usize, row: usize) -> bool {
        let mut all_true = true;
        for r in 0..5 {
            match self.board[col][r] {
                BoardSlot::Unmarked(_v) => {
                    all_true = false;
                    break;
                },
                _ => {}
            }
        }
        if all_true {
            return true;
        }

        all_true = true;
        for c in 0..5 {
            match self.board[c][row] {
                BoardSlot::Unmarked(_v) => {
                    all_true = false;
                    break;
                },
                _ => {}
            }
        }
        return all_true;
    }

    fn values(&self) -> impl Iterator<Item = BoardSlot> + '_ {
        let mut idx = 0;
        return std::iter::from_fn(move || {
            if idx >= 25 {
                return None;
            }
            let slot = self.board[idx % 5][idx / 5];
            idx += 1;
            return Some(slot);
        });
    }

}

impl BoardSlot {
    fn is_equal(&self, value: i32) -> bool {
        match self {
            BoardSlot::Unmarked(v) => {
                if *v == value {
                    return true;
                } else {
                    return false;
                }
            },
            _ => { return false; }
        }
    }
}

fn vec_to_array<T>(v: Vec<T>) -> [T; 5] where T: Copy {
    let slice = v.as_slice();
    let array: [T; 5] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 5, v.len()),
    };
    array
}

fn parse_boards(input: &str) -> Vec<Board> {
    let lines: Vec<&str> = input
        .lines()
        .skip(2)
        .filter(|line| line.to_string() != "")
        .collect();
    let mut boards: Vec<Board> = vec![];
    for chunk in lines.chunks(5) {
        let marked_board: Board = Board {
            board: vec_to_array(chunk
            .iter()
            .map(|line| {
                vec_to_array(line.split_whitespace()
                    .collect::<Vec<&str>>() .iter()
                    .map(|c| BoardSlot::Unmarked(c.parse::<i32>().unwrap()))
                    .collect())
            })
            .collect())};
        boards.push(marked_board);
    }
    return boards;

}

pub fn parse_input(input: &str) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<i32> = lines[0]
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    return numbers;
}

fn part1(input: &str) -> i32 {
    let mut winner = 0;
    let numbers = parse_input(&input);
    let mut boards = parse_boards(&input);
    'outer: for num in numbers {
        for idx in 0..boards.len() {
            let mark = boards[idx].mark(num);
            if mark {
                winner = num * boards[idx].values().map(|e| match e {
                    BoardSlot::Unmarked(v) => v,
                    _ => 0,
                }).sum::<i32>();
                println!("winner board: {}", idx);
                break 'outer;
            }
        }
    };
    return winner;
}


fn main() {
    const DATA: &str = include_str!("./day4.input");
    println!("{}", part1(&DATA));
}
