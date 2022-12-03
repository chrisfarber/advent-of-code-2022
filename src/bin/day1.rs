use std::{env, num::ParseIntError, process::exit};

use aoc::data::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(file_path) = args.get(1) else {
        eprintln!("Usage: day1 path/to/data.txt");
        exit(1);
    };

    day1(&file_path);
}

fn day1(path_str: &String) {
    let lines =
        read_lines(&path_str, |line| parse_int_or_empty(&line)).expect("could not parse file");
    let mut gnomes = group_calories(lines);
    let most = gnomes.iter().max();
    println!(
        "The gnome with the most food has {} calories",
        most.expect("should have been at least one?")
    );

    gnomes.sort();
    println!("the top three gnomes are:");
    let sum: u64 = gnomes.iter().rev().take(3).sum();
    println!("the sum is {}", sum);
}

fn group_calories(data: Vec<Line<u64>>) -> Vec<u64> {
    let mut gnome_totals: Vec<u64> = vec![];
    let mut current_sum: Option<u64> = None;
    let mut process = |l: Line<u64>| match l {
        Line::Blank => {
            if let Some(sum) = current_sum {
                gnome_totals.push(sum);
                current_sum = None;
            }
        }
        Line::Value(item) => current_sum = current_sum.map(|sum| sum + item).or(Some(item)),
    };
    for entry in data {
        process(entry)
    }
    process(Line::Blank);
    gnome_totals
}

#[derive(Debug)]
enum Line<T> {
    Value(T),
    Blank,
}
fn parse_int_or_empty(from_str: &String) -> Result<Line<u64>, ParseIntError> {
    if from_str == "" {
        return Ok(Line::Blank);
    }
    from_str.parse::<u64>().map(|v| Line::Value(v))
}
