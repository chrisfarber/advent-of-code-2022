use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Read a file into a vec of parsed items. Will panic on IO error.
pub fn read_lines<P, F, L, E>(filename: P, parse_line: F) -> Result<Vec<L>, E>
where
    P: AsRef<Path>,
    F: Fn(String) -> Result<L, E>,
    E: std::error::Error,
{
    let file = File::open(filename).expect("could not open file");
    let mut out: Vec<L> = vec![];
    for line in io::BufReader::new(file).lines() {
        let parsed = parse_line(line.expect("error reading line"));
        out.push(parsed?);
    }
    Ok(out)
}
