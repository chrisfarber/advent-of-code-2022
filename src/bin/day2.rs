use aoc::data::read_lines;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

pub fn main() {
    let data = read_lines("inputs/day2.txt", Round::parse).expect("could not parse the data?");
    let sum: u32 = data.iter().map(|r| r.score()).sum();
    println!("Final score: {}", sum);
}

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn against(&self, other: &Self) -> Outcome {
        match (self, other) {
            (a, b) if a == b => Outcome::Draw,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(Debug)]

struct Round {
    pub opponent: Shape,
    pub player: Shape,
}

static ROUND_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*([ABC])\s+([XYZ])\s*$").unwrap());

impl Round {
    fn parse(str: String) -> Result<Self, ParseError> {
        if let Some(captures) = ROUND_REGEX.captures(&str) {
            let opponent = match &captures[1] {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!("impossible capture?"),
            };
            let player = match &captures[2] {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!("impossible capture?"),
            };
            Ok(Round { opponent, player })
        } else {
            Err(ParseError::NotMatched(str))
        }
    }

    fn score(&self) -> u32 {
        self.player.value() + self.player.against(&self.opponent).value()
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("line did not match '{0}'")]
    NotMatched(String),
}

#[cfg(test)]
mod tests {
    use crate::ROUND_REGEX;

    #[test]
    fn parsing_regex() {
        let matched = ROUND_REGEX.is_match("nope");
        assert!(!matched, "this should not match");
        assert!(ROUND_REGEX.is_match("A Y"), "this should match");
        assert!(
            ROUND_REGEX.is_match("       A                          Y      \t "),
            "spaces are ignored"
        );
        assert!(!ROUND_REGEX.is_match("AA Y"), "repetitions not allowed");

        let captures = ROUND_REGEX.captures("      A    X").unwrap();
        assert_eq!(&captures[1], "A", "first capture is A");
        assert_eq!(&captures[2], "X", "second capture is X");

        let captures = ROUND_REGEX.captures("not gonna happen");
        assert!(captures.is_none(), "should give None if we don't match");
    }
}
