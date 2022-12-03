use std::collections::HashSet;

use aoc::data::read_lines;
use thiserror::Error;

#[derive(Error, Debug)]
enum Day3Error {
    #[error("a parse error")]
    ParseError {
        #[from]
        err: ParseError,
    },
}

fn main() -> Result<(), Day3Error> {
    let data = read_lines("inputs/day3.txt", |l| parse_line(l))?;
    let total: u32 = data
        .iter()
        .map(|r| r.common_types().iter().map(assign_value).sum::<u32>())
        .sum();
    println!("wat {}", total);
    Ok(())
}

fn assign_value(c: &u8) -> u32 {
    match *c {
        b'a'..=b'z' => 1 + (c - b'a'),
        b'A'..=b'Z' => 27 + (c - b'A'),
        _ => {
            panic!("bad input {}", c);
        }
    }
    .into()
}

#[derive(Debug)]
struct Rucksack {
    left: HashSet<u8>,
    right: HashSet<u8>,
}

impl Rucksack {
    pub fn common_types(&self) -> Vec<u8> {
        self.left
            .intersection(&self.right)
            .into_iter()
            .map(|v| *v)
            .collect()
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("the rucksack doesn't have an even length {0}")]
    NotEven(String),
}

fn parse_line(l: String) -> Result<Rucksack, ParseError> {
    let cs: Vec<u8> = l.as_bytes().into();
    if cs.len() % 2 != 0 {
        return Err(ParseError::NotEven(l));
    }
    let part_size = cs.len() / 2;
    let mut left = HashSet::new();
    let mut right = HashSet::new();
    for (i, c) in cs.iter().enumerate() {
        (if i < part_size { &mut left } else { &mut right }).insert(*c);
    }
    Ok(Rucksack { left, right })
}
