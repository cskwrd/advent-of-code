use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    println!("Total overlapping assignments: '{}'", solve(input_lines));
    
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

fn parse_cleaning_assignments(cleaning_assignment: &str) -> (HashSet<u32>, HashSet<u32>) {
    let unparsed_assignments = cleaning_assignment.split(',').collect::<Vec<&str>>();
    let (left_unparsed_assignment, right_unparsed_assignment) = (unparsed_assignments[0], unparsed_assignments[1]);
    let left_assignment: HashSet<u32> = parse_assignment(left_unparsed_assignment);
    let right_assignment: HashSet<u32> = parse_assignment(right_unparsed_assignment);
    (left_assignment, right_assignment)
}

fn parse_assignment(unparsed_assignment: &str) -> HashSet<u32> {
    let assignment_bounds = unparsed_assignment.split('-').collect::<Vec<&str>>();
    let (assignment_start, assignment_end) = (assignment_bounds[0].parse().expect("assignment start must be valid number"), assignment_bounds[1].parse().expect("assignment end must be valid number"));
    let mut assignment = HashSet::<u32>::new();
    for i in assignment_start..=assignment_end {
        assignment.insert(i);
    }
    assignment
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_assignment() {
        assert_eq!(parse_assignment("1-5"), HashSet::<u32>::from([1,2,3,4,5]));
    }

    #[test]
    fn test_solver() {
        // Please note, that private functions can be tested too!
        let input_lines = include_str!("../../../aoc-example-1.txt").lines();
        assert_eq!(solve(input_lines), 4);
    }
}
