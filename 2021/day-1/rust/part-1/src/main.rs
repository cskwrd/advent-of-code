use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n"); // this returns an iterator, also splitting on "\n" doesn't work for files using CRLF

    let mut previous_depth = -1;
    let mut positive_measurements = -1;
    for line in lines {
        let depth: i32 = line.parse().unwrap();

        if depth > previous_depth {
            positive_measurements += 1;
        }
        previous_depth = depth;
    }

    println!("In this example, there are '{}' measurements that are larger than the previous measurement.", positive_measurements);
}
