use std::{collections::HashMap, path::{Path, PathBuf}};

use nom::{branch::alt, bytes::complete::{take_while1, tag}, combinator::{map, all_consuming}, Finish, IResult, sequence::{preceded, separated_pair}};

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(PathBuf),
    File(u32, PathBuf),
}

fn parse_path(path: &str) -> IResult<&str, PathBuf> {
    map(
        take_while1(|c| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into
    )(path)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u32, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
        .collect();

    println!("dir size: '{}'", solve(input_lines, 100000));
    
    Ok(())
}

fn solve(input_lines: Vec<Line>, max_size: u32) -> u32 {
    let mut working_dir = Path::new(".").to_path_buf();
    let mut du = HashMap::<PathBuf, u32>::new(); // du, like the command
    for line in input_lines {
        match line {
            Line::Command(command) => match command {
                Command::Cd(_test) => {
                    let mut child_dir_size = 0u32;
                    match _test.to_str().unwrap() {
                        ".." => {
                            if let Some(disk_space_used) = du.get(&working_dir) {
                                child_dir_size = *disk_space_used;
                            }
                            working_dir.pop();
                        }
                        _ => {
                            working_dir.push(_test);
                        }
                    };
                    
                    if let Some(disk_space_used) = du.get_mut(&working_dir) {
                        *disk_space_used += child_dir_size;
                    } else {
                        du.insert(working_dir.clone(), 0u32);
                    }
                }
                Command::Ls => {
                    // do nothing with this command, it doesn't really change anything in the big picture
                },
            }
            Line::Entry(entry) => match entry {
                Entry::Dir(_dir) => {
                    // do nothing with this type of entry, it doesn't really change anything in the big picture
                }
                Entry::File(size, _file) => {
                    if let Some(disk_space_used) = du.get_mut(&working_dir) {
                        *disk_space_used += size;
                    }
                }
            },
        }
    }
    println!("du: {:?}", du);
    du.values().filter(|size| **size <= max_size).sum()
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solver_1() {
        // Please note, that private functions can be tested too!
        let input_lines = include_str!("../../../aoc-example-1.txt")
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect();

        assert_eq!(solve(input_lines, 100000), 95437u32);
    }
}
