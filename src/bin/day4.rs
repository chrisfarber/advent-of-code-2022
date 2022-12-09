use aoc::data::read_lines;
use nom::sequence::separated_pair;
use thiserror::Error;

fn main() {
    println!("day 4");
    let data = read_lines("inputs/day4.txt", |l| {
        let wat = parse_assignment_pair(&l);
        let ok = wat.map(|(_, res)| res);
        ok.map_err(|_| ParseError::Bad("bleh".to_string()))
    })
    .expect("something didn't parse");
    println!("first? {:?}", data[0]);

    let contained = data
        .iter()
        .filter(|pair| pairs_fully_overlap(&pair.0, &pair.1))
        .count();
    println!("there are {} fully contained pairs", contained);

    let overlap = data.iter().filter(|pair| pairs_overlap(pair)).count();
    println!("there are {} that overlap at all", overlap);
}

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

fn pairs_fully_overlap(a: &Assignment, b: &Assignment) -> bool {
    (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

fn pairs_overlap((a, b): &(Assignment, Assignment)) -> bool {
    (a.start <= b.start && b.start <= a.end) || (b.start <= a.start && a.start <= b.end)
}

fn parse_assignment(i: &str) -> nom::IResult<&str, Assignment> {
    separated_pair(
        nom::character::complete::u32,
        nom::bytes::complete::tag("-"),
        nom::character::complete::u32,
    )(i)
    .map(|(rest, (start, end))| (rest, Assignment { start, end }))
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("not fun")]
    Bad(String),
}

fn parse_assignment_pair(i: &str) -> nom::IResult<&str, (Assignment, Assignment)> {
    separated_pair(
        parse_assignment,
        nom::bytes::complete::tag(","),
        parse_assignment,
    )(i)
}
