use std::collections::{HashSet, VecDeque};

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    let datastream = get_datastream_from_input(input_lines);

    println!("Start-of-packet marker after num characters: '{}'", solve(datastream, 14));
    
    Ok(())
}

fn solve(datastream: String, checksum_length: usize) -> u32 {
    if datastream.len() < checksum_length {
        panic!("datastream not big enough");
    }
    
    let mut start_of_packet_marker = VecDeque::<char>::new();
    let mut num_chars_processed = 0u32;
    for (i, chr) in datastream.chars().enumerate() {
        start_of_packet_marker.push_back(chr);
        if HashSet::<char>::from_iter(start_of_packet_marker.clone().into_iter()).len() == checksum_length {
            num_chars_processed = i as u32 + 1;
            break;
        }
        if start_of_packet_marker.len() >= checksum_length {
            start_of_packet_marker.pop_front();
        }
    }

    if num_chars_processed == 0u32 {
        panic!("invalid datastream, no matching sequence");
    }
    
    num_chars_processed
}

fn get_datastream_from_input(mut input_lines: std::str::Lines) -> String {
    input_lines.next().expect("cannot read input").to_owned() // using `.to_owned` because we don't want to borrow the string we want a copy of it
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solver_1() {
        // Please note, that private functions can be tested too!
        let input_lines = include_str!("../../../aoc-example-1.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream, 14), 19u32);
    }

    #[test]
    fn test_solver_2() {
        let input_lines = include_str!("../../../aoc-example-2.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream, 14), 23u32);
    }

    #[test]
    fn test_solver_3() {
        let input_lines = include_str!("../../../aoc-example-3.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream, 14), 23u32);
    }

    #[test]
    fn test_solver_4() {
        let input_lines = include_str!("../../../aoc-example-4.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream, 14), 29u32);
    }

    #[test]
    fn test_solver_5() {
        let input_lines = include_str!("../../../aoc-example-5.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream, 14), 26u32);
    }
}
