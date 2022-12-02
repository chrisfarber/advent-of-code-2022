use std::{
    env,
    fs::File,
    io::{self, BufRead, Error},
    path::Path,
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(file_path) = args.get(1) else {
        eprintln!("Usage: day1 path/to/data.txt");
        exit(1);
    };

    day1(&file_path).unwrap();
}

fn day1(path_str: &String) -> Result<(), Error> {
    let lines = read_lines(&path_str)?
        .map(|l| l.unwrap())
        .map(|l| parse_int_or_empty(&l).unwrap());
    for line in lines {
        println!("{:?}", line);
    }
    Ok(())
}

#[derive(Debug)]
enum Line<T> {
    Value(T),
    Blank,
}
fn parse_int_or_empty(from_str: &String) -> Result<Line<u64>, ()> {
    if from_str == "" {
        return Ok(Line::Blank);
    }
    if let Ok(value) = from_str.parse::<u64>() {
        return Ok(Line::Value(value));
    }
    return Err(());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
