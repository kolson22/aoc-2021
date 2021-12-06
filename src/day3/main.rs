#![feature(drain_filter)]
use std::{cmp::Ordering, ops::Add, str::FromStr, string::ParseError};

#[derive(Copy, Clone)]
enum Mode {
    MostCommon,
    LeastCommon,
}

fn parse_input(input: &str) -> Result<Vec<String>, ParseError> {
    input.lines().map(FromStr::from_str).collect()
}

fn part1(numbers: &[String]) -> usize {
    let mut final_bin = String::new();
    for i in 0..numbers[0].len() {
        let count_one = numbers.iter().filter(|x| x[i..].starts_with('1')).count();
        let count_zero = numbers.len() - count_one;

        let current_bit_mask = match count_one.cmp(&count_zero) {
            Ordering::Less => '0',
            Ordering::Equal | Ordering::Greater => '1',
        };

        final_bin.push(current_bit_mask);
    }

    let inv_final_bin = final_bin.chars().fold(String::new(), |inv_final_bin, c| {
        if c == '1' {
            inv_final_bin.add("0")
        } else {
            inv_final_bin.add("1")
        }
    });
    let gamma_rate = usize::from_str_radix(&final_bin, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&inv_final_bin, 2).unwrap();
    gamma_rate * epsilon_rate
}


fn search_value(numbers: &[String], mode: Mode) -> usize {
    let critical_bit = match mode {
        Mode::MostCommon => '1',
        Mode::LeastCommon => '0',
    };

    let mut local_numbers: Vec<&String> = numbers.iter().collect();

    for i in 0..numbers[0].len() {
        if local_numbers.len() == 1 {
            break;
        }
        let count_one = local_numbers
            .iter()
            .filter(|x| x[i..].starts_with(critical_bit))
            .count();
        let count_zero = local_numbers.len() - count_one;

        let current_bit_mask = match count_one.cmp(&count_zero) {
            Ordering::Less => '0',
            Ordering::Equal => critical_bit,
            Ordering::Greater => '1',
        };

        local_numbers.drain_filter(|x| x[i..].starts_with(current_bit_mask));
    }

    usize::from_str_radix(local_numbers[0], 2).unwrap()
}

fn part2(numbers: &[String]) -> usize {
    let oxygen = search_value(numbers, Mode::MostCommon); // Get Oxygen Generator Rating
    let co2 = search_value(numbers, Mode::LeastCommon); // Get CO2 Scrubber Rating
    oxygen * co2
}

fn main() {
    const DATA: &str = include_str!("./day3.input");
    let result = part1(&parse_input(DATA).unwrap());
    println!("The answer for part1 is: {}", result);
    let result2 = part2(&parse_input(DATA).unwrap());
    println!("The answer for part2 is: {}", result2);
}
