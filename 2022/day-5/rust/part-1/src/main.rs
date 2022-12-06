use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    println!("Top of stacks: '{}'", solve(input_lines));
    
    Ok(())
}

fn solve(input_lines: std::str::Lines) -> u32 {
    let mut overlapping_assignments = 0;
    for (first_elf_assignment, right_elf_assignment) in input_lines.map(parse_cleaning_assignments) {
        let overlap_size = first_elf_assignment.intersection(&right_elf_assignment).collect::<HashSet<&u32>>().len();
        if overlap_size > 0 {
            overlapping_assignments += 1;
        }
    }
    overlapping_assignments
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solver() {
        // Please note, that private functions can be tested too!
        let input_lines = include_str!("../../../aoc-example-1.txt").lines();
        assert_eq!(solve(input_lines), 4);
    }
}
