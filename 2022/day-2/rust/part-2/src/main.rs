fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();
    
    let mut projected_score = 0;
    for line in input_lines {
        // first column is opponent, second is match result
        // code:
        //     shape:
        //         A: Rock
        //         B: Paper
        //         C: Scissors
        //     result:
        //         X: Lose
        //         Y: Draw
        //         Z: Win
        // scoring:
        //     shape:
        //         Rock: 1
        //         Paper: 2
        //         Scissors: 3
        //     outcome:
        //         Win: 6
        //         Lose: 0
        //         Draw: 3
        projected_score += match line {
            "A X" => 3, // Lose vs. Rock
            "A Y" => 4, // Draw vs. Rock
            "A Z" => 8, // Win  vs. Rock
            "B X" => 1, // Lose vs. Paper
            "B Y" => 5, // Draw vs. Paper
            "B Z" => 9, // Win  vs. Paper
            "C X" => 2, // Lose vs. Scissors
            "C Y" => 6, // Draw vs. Scissors
            "C Z" => 7, // Win  vs. Scissors
            _ => panic!("Unhandled: '{}'", line),
        };
    }

    println!("Projected score: '{}'", projected_score);

    Ok(())
}
