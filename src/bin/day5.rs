use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, newline, satisfy, space0, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
    ToUsize,
};

fn main() {
    let f = File::open("inputs/day5.txt").expect("File should exist");
    let mut buf_read = BufReader::new(f);
    let mut text = String::new();
    buf_read
        .read_to_string(&mut text)
        .expect("failed to read file");

    let (rest, (crate_def, crate_indexes, moves)) = parse_file(&text).expect("didn't parse ok");
    println!("received?\n{:?}", crate_def);
    let mut stack = CrateStacks::construct(&crate_def).expect("it should work!");
    let mut stack2 = CrateStacks::construct(&crate_def).expect("it should work");
    println!("got out?\n{:?}", stack);
    for a_move in moves {
        stack.apply_move(&a_move);
        stack2.apply_move_with_multiple_crates(&a_move);
    }
    println!("read out the message {}", stack.message());
    println!("moving multiple: {}", stack2.message());
}

#[derive(Debug)]
struct CrateStacks {
    stacks: Vec<Vec<Crated>>,
}

impl CrateStacks {
    fn construct(from: &Vec<Vec<Option<Crated>>>) -> Option<Self> {
        let width = from[0].len();
        if !from.iter().map(|r| r.len()).all(|len| len == width) {
            return None;
        }
        let mut stacks = Vec::with_capacity(width);
        (0..width).for_each(|_| {
            stacks.push(Vec::with_capacity(from.len()));
        });

        for line in from.iter().rev() {
            for (i, c) in line.iter().enumerate() {
                if let Some(c) = c {
                    stacks[i].push(c.clone())
                }
            }
        }
        return Some(Self { stacks });
    }

    fn apply_move(&mut self, the_move: &Move) {
        let from = (the_move.from_index - 1).to_usize();
        let to = (the_move.to_index - 1).to_usize();
        for _ in 0..the_move.count {
            if let Some(v) = self.stacks[from].pop() {
                self.stacks[to].push(v);
            }
        }
    }

    fn apply_move_with_multiple_crates(&mut self, the_move: &Move) {
        let from_idx = (the_move.from_index - 1).to_usize();
        let to_idx = (the_move.to_index - 1).to_usize();

        let from = &mut self.stacks[from_idx];
        let mut moved = from.split_off(0.max(from.len() - the_move.count.to_usize()));

        self.stacks[to_idx].append(&mut moved);
    }

    fn message(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last())
            .flatten()
            .map(|c| c.letter)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Crated {
    letter: char,
}

fn parse_crate(i: &str) -> nom::IResult<&str, Crated> {
    delimited(tag("["), one_alpha, tag("]"))(i).map(|(r, c)| (r, Crated { letter: c }))
}

fn one_alpha(i: &str) -> nom::IResult<&str, char> {
    satisfy(|c| c.is_ascii_alphabetic())(i)
}

fn parse_crate_or_space(i: &str) -> nom::IResult<&str, Option<Crated>> {
    nom::branch::alt((
        nom::combinator::map(parse_crate, |c| Some(c)),
        nom::combinator::map(tag("   "), |_| None),
    ))(i)
}

fn parse_crates_line(i: &str) -> nom::IResult<&str, Vec<Option<Crated>>> {
    terminated(
        separated_list1(nom::character::complete::char(' '), parse_crate_or_space),
        nom::character::complete::line_ending,
    )(i)
}

fn parse_crate_index(i: &str) -> nom::IResult<&str, u8> {
    delimited(
        tag(" "),
        nom::character::complete::u8,
        nom::combinator::opt(tag(" ")),
    )(i)
}

fn parse_crate_indexes_line(i: &str) -> nom::IResult<&str, Vec<u8>> {
    terminated(
        separated_list1(tag(" "), parse_crate_index),
        nom::character::complete::line_ending,
    )(i)
}

fn parse_file(i: &str) -> nom::IResult<&str, (Vec<Vec<Option<Crated>>>, Vec<u8>, Vec<Move>)> {
    nom::sequence::tuple((
        many1(parse_crates_line),
        terminated(parse_crate_indexes_line, line_ending),
        many1(parse_move),
    ))(i)
}

#[derive(Debug, PartialEq)]
struct Move {
    count: u8,
    from_index: u8,
    to_index: u8,
}

fn parse_move(i: &str) -> nom::IResult<&str, Move> {
    nom::combinator::map(
        nom::sequence::tuple((
            tag("move"),
            space1,
            nom::character::complete::u8,
            space1,
            tag("from"),
            space1,
            nom::character::complete::u8,
            space1,
            tag("to"),
            space1,
            nom::character::complete::u8,
            nom::combinator::opt(space0),
            nom::combinator::opt(line_ending),
        )),
        |(_, _, count, _, _, _, from_index, _, _, _, to_index, _, _)| Move {
            count,
            from_index,
            to_index,
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use crate::{parse_crate_indexes_line, parse_move, Move};

    #[test]
    fn parsing_indexes() {
        assert_eq!(
            parse_crate_indexes_line(" 1   2   3 \n").unwrap(),
            ("", vec![1, 2, 3])
        )
    }

    #[test]
    fn parsing_moves() {
        let (_, move1) = parse_move("move 7 from 8 to 9").unwrap();
        assert_eq!(
            move1,
            Move {
                count: 7,
                from_index: 8,
                to_index: 9
            }
        );

        parse_move("move 1 from 1 to 1             \n").unwrap();
    }
}
