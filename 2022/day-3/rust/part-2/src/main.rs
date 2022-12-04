use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    let mut overall_priorities = 0;
    let mut group = HashSet::<char>::new();
    for (i, rucksack) in input_lines.enumerate() {
        if group.is_empty() {
            group = rucksack.chars().collect::<HashSet<char>>();
        } else {
            let mut new_group = HashSet::<char>::new();
            group.intersection(&rucksack.chars().collect::<HashSet<char>>()).for_each(|intersected_item| { new_group.insert(*intersected_item); }); // i don't yet understand: https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy
            group = new_group;
        }

        if (i + 1) % 3 == 0 {
            if group.len() == 1 {
                let group_indicator = group.clone().into_iter().next().expect("error getting indicator");
                println!("Group indicator: '{}'", group_indicator);
                overall_priorities += get_item_priority(group_indicator);
                group.clear();
            } else {
                panic!("every group needs an indicator");
            }
        }
    }

    println!("Priorities: '{}'", overall_priorities);

    Ok(())
}

fn get_item_priority(item: char) -> u32 {
    let base_priority = if item.is_ascii_uppercase() {
        38
    } else {
        96
    };
    u32::from(item) - base_priority
}
