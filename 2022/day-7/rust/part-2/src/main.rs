use std::{collections::HashMap, path::{Path, PathBuf}};

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    println!("dir size: '{}'", solve(input_lines, 100000));
    
    Ok(())
}

fn solve(input_lines: std::str::Lines, max_size: u32) -> u32 {
    let mut working_dir = Path::new(".").to_path_buf();
    let mut du = HashMap::<PathBuf, u32>::new(); // du, like the command
    for line in input_lines {
        if line.starts_with("$ ") {
            let cmd_line = line.strip_prefix("$ ").expect("invalid cmd line");
            if cmd_line.starts_with("cd ") {
                let new_dir = cmd_line.strip_prefix("cd ").expect("invalid cd cmd");
                let mut child_dir_size = 0u32;
                if new_dir != ".." {
                    working_dir.push(new_dir);
                } else {
                    if let Some(disk_space_used) = du.get(&working_dir) {
                        child_dir_size = *disk_space_used;
                    }
                    working_dir.pop();
                }
                if let Some(disk_space_used) = du.get_mut(&working_dir) {
                    *disk_space_used += child_dir_size;
                } else {
                    du.insert(working_dir.clone(), 0u32);
                }
                println!("pwd: '{:?}'", working_dir);
            }
        } else if !line.starts_with("dir ") {
            let (size_str, file_name) = line.split_once(' ').expect("unable to parse file listing");
            let file_size = size_str.parse::<u32>().expect("unable to parse file size");
            
            println!("file: '{}' (size: {})", file_name, file_size);
            
            if let Some(disk_space_used) = du.get_mut(&working_dir) {
                *disk_space_used += file_size;
            }
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
        let input_lines = include_str!("../../../aoc-example-1.txt").lines();//.collect::<Vec<_>>();

        // let thing = input_lines.split(|line| line.starts_with('$')).collect::<Vec<_>>();

        // println!("split lines: {:?}", thing);

        assert_eq!(solve(input_lines, 100000), 95437u32);
    }
}
