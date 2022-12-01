fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();
    
    let mut most_cals: Vec<i64> = vec![0; 3]; // create vector with 3 elements intialized to 0
    let mut cals_in_pack: i64 = 0;
    for line in input_lines {
        if !line.is_empty() {
            // add calories
            let calories = str::parse::<i64>(line)?;
            cals_in_pack += calories;
        } else {
            // moving to next elf, update tacker
            if cals_in_pack > most_cals[0] {
                most_cals[2] = most_cals[1];
                most_cals[1] = most_cals[0];
                most_cals[0] = cals_in_pack;
            } else if cals_in_pack > most_cals[1] {
                most_cals[2] = most_cals[1];
                most_cals[1] = cals_in_pack;
            } else if cals_in_pack > most_cals[2] {
                most_cals[2] = cals_in_pack;
            }
            cals_in_pack = 0;
        }
    }

    println!("Most calories: {}", most_cals.iter().sum::<i64>());

    Ok(())
}
