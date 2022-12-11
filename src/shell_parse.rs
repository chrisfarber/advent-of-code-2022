use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{line_ending, space0, space1},
    combinator::{eof, map, not, opt, peek, recognize},
    multi::many_till,
    sequence::{preceded, terminated, tuple},
    IResult,
};

type Res<'a, T> = IResult<&'a str, T>;
type StrRes<'a> = Res<'a, &'a str>;

/// parses text until a newline. consumes the newline, but does not return
/// it in its output.
pub fn until_eol(i: &str) -> StrRes {
    terminated(
        take_while(|b| b != '\r' && b != '\n'),
        alt((line_ending, eof)),
    )(i)
}

/// Extracts a shell command from the input.
/// A shell command starts with "$ " and ends with a newline character.
///
/// The match will be everything in between.
pub fn shell_cmd(i: &str) -> StrRes {
    preceded(tag("$ "), until_eol)(i)
}

pub fn not_shell_cmd(i: &str) -> StrRes {
    preceded(not(tag("$ ")), until_eol)(i)
}

pub fn shell_cmd_and_output(i: &str) -> IResult<&str, (&str, &str)> {
    tuple((
        shell_cmd,
        recognize(many_till(not_shell_cmd, peek(alt((tag("$ "), eof))))),
    ))(i)
}

/// Parse a cd command.
///
/// Given the input "cd hello", the output will be "hello"
fn parse_cd_cmd(i: &str) -> StrRes {
    preceded(tag("cd "), until_eol)(i)
}

pub fn cd_cmd(i: &str) -> Option<&str> {
    parse_cd_cmd(i).map(|(_, dir)| dir).ok()
}

fn parse_ls_cmd(i: &str) -> IResult<&str, ()> {
    map(
        tuple((tag("ls"), opt(space0), alt((line_ending, eof)))),
        |_| (),
    )(i)
}

type FileLine<'a> = (&'a str, u64);
fn parse_ls_out_file_line(i: &str) -> IResult<&str, FileLine> {
    map(
        tuple((nom::character::complete::u64, space1, until_eol)),
        |(size, _, name)| (name, size),
    )(i)
}

pub fn ls_cmd(i: &str) -> bool {
    parse_ls_cmd(i).is_ok()
}

pub fn commands<'a>(from: &'a str) -> CmdResponseIterator<'a> {
    CmdResponseIterator { rest: from }
}

pub struct CmdResponseIterator<'a> {
    rest: &'a str,
}

pub struct CommandResponse<'a> {
    pub command: &'a str,
    pub output: &'a str,
}

impl<'a> Iterator for CmdResponseIterator<'a> {
    type Item = CommandResponse<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.rest == "" {
            return None;
        }
        if let Ok((rest, (command, output))) = shell_cmd_and_output(self.rest) {
            self.rest = rest;
            return Some(CommandResponse { command, output });
        }
        panic!("should have reached the end.");
    }
}

pub struct LsFileIterator<'a> {
    rest: &'a str,
}

impl<'a> Iterator for LsFileIterator<'a> {
    type Item = (&'a str, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest == "" {
            return None;
        }
        let res = parse_ls_out_file_line(self.rest);
        if let Ok((rest, parsed)) = res {
            self.rest = rest;
            Some(parsed)
        } else {
            let maybe_skip = until_eol(self.rest);
            if let Ok((rest, line)) = maybe_skip {
                self.rest = rest;
                if line.starts_with("dir ") {
                    return self.next();
                }
            }
            panic!("Unexpected input found parsing ls output, at {:?}", res);
        }
    }
}

/// Iterate over ONLY the files.
/// Lines that indicate subfolders will be skipped
pub fn ls_out_files(i: &str) -> LsFileIterator {
    LsFileIterator { rest: i }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_command_parsing() {
        let huh = shell_cmd("this should not match");
        assert!(huh.is_err());

        assert!(shell_cmd("$ eh").is_ok());
        let a_command = shell_cmd("$ cd ..\n");
        assert_eq!("cd ..", a_command.unwrap().1);

        let (rest, m) = not_shell_cmd("hello\r\n$ woah").unwrap();
        assert_eq!("hello", m);
        assert_eq!("$ woah", rest);

        let (rest, out) =
            not_shell_cmd("hello there this is not shell\r\nbut this is also not\r\n$ but this is")
                .unwrap();
        assert_eq!(out, "hello there this is not shell");

        let (rest, out) = not_shell_cmd(rest).unwrap();
        assert_eq!(out, "but this is also not");

        let (rest, out) = not_shell_cmd("eh").unwrap();
        assert_eq!(out, "eh");
    }

    #[test]
    fn shell_command_and_output_test() {
        let input = "$ ls
line 1
line 2";
        println!("input?\n{}", input);
        let (rest, (cmd, out)) = shell_cmd_and_output(input).unwrap();
        assert_eq!("ls", cmd);
        assert_eq!(
            out,
            "line 1
line 2"
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn many_shell_command_and_output_test() {
        let input = "$ ls
a
b
$ cd ..
c
d
$ ls";
        let (rest, (cmd, out)) = shell_cmd_and_output(input).unwrap();
        assert_eq!(cmd, "ls");
        assert_eq!(out, "a\nb\n");

        let (rest, (cmd, out)) = shell_cmd_and_output(rest).unwrap();
        assert_eq!(cmd, "cd ..");
        assert_eq!(out, "c\nd\n");

        let (rest, (cmd, out)) = shell_cmd_and_output(rest).unwrap();
        assert_eq!(cmd, "ls");
        assert_eq!(out, "");

        assert_eq!(rest, "");
    }

    #[test]
    fn test_cmd_iter() {
        let input = "\
$ hello yes
is this dog?
well,
yeah sure is.
$ twice now

$ thrice";
        let mut iter = commands(input);
        let first = iter.next().unwrap();
        assert_eq!(first.command, "hello yes");
        assert_eq!(first.output, "is this dog?\nwell,\nyeah sure is.\n");

        let second = iter.next().unwrap();
        assert_eq!(second.command, "twice now");
        assert_eq!(second.output, "\n");

        let third = iter.next().unwrap();
        assert_eq!(third.command, "thrice");
        assert_eq!(third.output, "");

        assert!(iter.next().is_none());
    }

    #[test]
    fn parsing_cd() {
        assert_eq!(
            "woah nice this ignores spaces",
            cd_cmd("cd woah nice this ignores spaces").unwrap()
        );

        assert_eq!("fancy", cd_cmd("cd fancy\nshit here").unwrap());

        assert!(cd_cmd("hello").is_none());
    }

    #[test]
    fn parsing_ls() {
        assert_eq!(true, ls_cmd("ls"));
        assert_eq!(true, ls_cmd("ls     \t "));
        assert_eq!(true, ls_cmd("ls\nwat"));
        assert_eq!(false, ls_cmd(""));
        assert_eq!(false, ls_cmd("ls fuck"));
    }

    #[test]
    fn parsing_ls_out_lines() {
        let mut iter = ls_out_files(
            "\
1234 somefile.txt
dir whocares
5678 otherfile.txt
",
        );
        assert_eq!(("somefile.txt", 1234), iter.next().unwrap());
        assert_eq!(("otherfile.txt", 5678), iter.next().unwrap());
        assert!(iter.next().is_none());
    }
}
