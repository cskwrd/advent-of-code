use std::collections::{HashSet, VecDeque};

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    let mut overall_priorities = 0;
    for rucksack in input_lines {
        if rucksack.len() % 2 == 0 {
            let mut items: VecDeque<char> = rucksack.chars().collect();
            let mut left_compartment = HashSet::<char>::new();
            let mut right_compartment = HashSet::<char>::new();
            while !items.is_empty() {
                let left_item = items.pop_front().expect("left item bad");
                left_compartment.insert(left_item);
                let right_item = items.pop_back().expect("right item bad");
                right_compartment.insert(right_item);
                if right_compartment.contains(&left_item) {
                    overall_priorities += get_item_priority(left_item);
                    break;
                } else if left_compartment.contains(&right_item) {
                    overall_priorities += get_item_priority(right_item);
                    break;
                }
            }
        } else {
            panic!("bad rucksack: not even");
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
