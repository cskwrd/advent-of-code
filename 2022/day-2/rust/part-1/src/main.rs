fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();
    
    let mut projected_score = 0;
    for line in input_lines {
        // first column is opponent, second is your move
        // A & X == Rock
        // B & Y == Paper
        // C & Z == Scissors
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
            "A X" => 4, // Draw w/ Rock
            "A Y" => 8, // Win  w/ Paper
            "A Z" => 3, // Lose w/ Scissors
            "B X" => 1, // Lose w/ Rock
            "B Y" => 5, // Draw w/ Paper
            "B Z" => 9, // Win  w/ Scissors
            "C X" => 7, // Win  w/ Rock
            "C Y" => 2, // Lose w/ Paper
            "C Z" => 6, // Draw w/ Scissors
            _ => panic!("Unhandled: '{}'", line),
        };
    }

    println!("Projected score: '{}'", projected_score);

    Ok(())
}
