fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    println!("Start-of-packet marker after num characters: '{}'", solve(input_lines, 100000));
    
    Ok(())
}

fn solve(input_lines: std::str::Lines, max_size: u32) -> u32 {
    0
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solver_1() {
        // Please note, that private functions can be tested too!
        let input_lines = include_str!("../../../aoc-example-1.txt").lines();

        assert_eq!(solve(input_lines, 100000), 95437u32);
    }
}
