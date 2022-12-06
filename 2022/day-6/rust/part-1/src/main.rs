fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    let datastream = get_datastream_from_input(input_lines);

    println!("Start-of-packet marker after num characters: '{}'", solve(datastream));
    
    Ok(())
}

fn solve(datastream: String) -> u32 {
    0
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

        assert_eq!(solve(datastream), 7u32);
    }

    #[test]
    fn test_solver_2() {
        let input_lines = include_str!("../../../aoc-example-2.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream), 5u32);
    }

    #[test]
    fn test_solver_3() {
        let input_lines = include_str!("../../../aoc-example-3.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream), 6u32);
    }

    #[test]
    fn test_solver_4() {
        let input_lines = include_str!("../../../aoc-example-4.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream), 10u32);
    }

    #[test]
    fn test_solver_5() {
        let input_lines = include_str!("../../../aoc-example-5.txt").lines();

        let datastream = get_datastream_from_input(input_lines);

        assert_eq!(solve(datastream), 11u32);
    }
}
