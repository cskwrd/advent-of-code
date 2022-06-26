use std::env;
use std::fs;

fn main() {
    use std::collections::HashMap;

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut expenses = HashMap::<i32, i32>::new();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split('\n'); // this returns an iterator, also splitting on "\n" doesn't work for files using CRLF

    for line in lines {
        let item: i32 = line.parse().unwrap();
        if expenses.contains_key(&item) {
            let other_item = expenses.get(&item).unwrap(); // expenses.get returns an Option<> type, seems to be similar to Nullable<>
            println!("{:?} * {:?} = {:?}", item, other_item, item * other_item);
        } else {
            expenses.insert(2020 - item, item); // add to hash map
        }
    }

    println!("All possible matches found!");
}
