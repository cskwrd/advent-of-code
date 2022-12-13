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
        take_while1(|c| "abcdefghijklmnopqrstuvwxyz./\\".contains(c)),
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
    let mut input_lines = include_str!("../../../aoc-input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    println!("dir size: '{}'", solve(&mut input_lines, 70000000u32, 30000000u32));
    
    Ok(())
}

fn solve(session_io_iterator: &mut dyn Iterator<Item = Line>, total_filesystem_size: u32, needed_free_space: u32) -> u32 {
    let mut working_dir = Path::new(".").to_path_buf();
    let mut du = HashMap::<PathBuf, u32>::new(); // du, like the command
    for line in session_io_iterator {
        match line {
            Line::Command(command) => match command {
                Command::Cd(dest) => {
                    let mut child_dir_size = 0u32;
                    match dest.to_str().unwrap() {
                        ".." => {
                            if let Some(disk_space_used) = du.get(&working_dir) {
                                child_dir_size = *disk_space_used;
                            }
                            working_dir.pop();
                        },
                        "/" => {
                            working_dir.push("\\");
                        },
                        _ => {
                            working_dir.push(dest);
                        },
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
    
    // roll up last size calculation
    loop {
        let working_dir_size = *du.get(&working_dir).expect("working dir must have size here");
        if let Some(disk_space_used) = du.get_mut(&(working_dir.parent().unwrap().to_path_buf())) {
            *disk_space_used += working_dir_size;
        }
        if !working_dir.pop() || working_dir.to_str().unwrap() == "\\" {
            break;
        }
    }
    
    // the following line assumes windows path
    let used_space = du.get(&(Path::new("\\").to_path_buf())).expect("working dir must have size here");
    let remaining_free_space = total_filesystem_size - used_space;
    
    du.values().filter(|size| **size + remaining_free_space >= needed_free_space).min().expect("a entry must be deleted").to_owned()
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solver_1() {
        // Please note, that private functions can be tested too!
        let mut input_lines = include_str!("../../../aoc-example-1.txt")
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

        assert_eq!(solve(&mut input_lines, 70000000u32, 30000000u32), 24933642u32);
    }
}
